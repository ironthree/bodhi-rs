// ! This module contains the structures and methods to interact with a (remote) bodhi server
// ! instance.

use std::time::Duration;

use fedora::{OpenIDSessionKind, Session};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::data::{FEDORA_BODHI_STG_URL, FEDORA_BODHI_URL};
use crate::error::{BodhiError, QueryError, ServiceError};
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};
use crate::CSRFQuery;

// This constant defines how many items are queried every time for multi-page queries. The
// server-side maximum is usually 100, the default is 20, and 50 seems a good compromise for speed.
pub const DEFAULT_ROWS: u32 = 50;

// Specify a longer timeout duration (60 s) for bodhi requests. The `reqwest` default value of 30
// seconds is a bit too short for long-running queries.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

// Specify a number of retries in case of connection or transient server failures.
const REQUEST_RETRIES: usize = 3;

// Specify a sane default user agent for bodhi-rs.
const USER_AGENT: &str = "bodhi-rs";

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum BodhiServiceType {
    DEFAULT,
    STAGING,
    CUSTOM { openid_url: String },
}

// This struct contains information necessary to build an instance of
// [`BodhiService`](struct.BodhiService.html) with the necessary flags. Additionally, depending on
// whether username and password were supplied as arguments, building the service instance will
// try to return a privileged session by authenticating via the fedora OpenID endpoint first.
//
// It's possible to target either the fedora production or the staging instances of bodhi, or
// provide a custom URL, via the `default()`, `staging()`, and `custom()` methods, respectively.
//
// ```
// // create service with anonymous session
// let bodhi = bodhi::BodhiServiceBuilder::default()
//     .timeout(std::time::Duration::from_secs(42))
//     .retries(9001)
//     .build();
// ```
//
// ```
// // builder for an authenticated session
// let builder = bodhi::BodhiServiceBuilder::staging()
//     .timeout(std::time::Duration::from_secs(120))
//     .retries(2)
//     .authentication("bodhi-rs", "password1");
// // builder.build();
// ```
#[derive(Debug)]
pub struct BodhiServiceBuilder<'a> {
    service_type: BodhiServiceType,
    authentication: Option<Authentication<'a>>,
    url: String,
    timeout: Option<Duration>,
    retries: Option<usize>,
}

#[derive(Debug)]
struct Authentication<'a> {
    username: &'a str,
    password: &'a str,
}

// This enum contains variants for all the ways in which constructing a
// [`BodhiService`](struct.BodhiService.html) instance can fail.
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    // This error represents an issue while parsing user-supplied URLs. If should never be returned
    // for the default settings, since the hardcoded URLs should always be valid.
    #[error("Failed to parse service URL: {error}")]
    UrlParsingError {
        // The inner error contains the issue that occurred while parsing the invalid URL.
        #[from]
        error: url::ParseError,
    },
    // This error represents an issue that occurred during authentication via the OpenID API.
    #[error("Failed to initialize OpenID client: {error}")]
    OpenIDClientError {
        // The inner error contains the issue that occurred during the set up of an authenticated
        // session via an OpenID endpoint.
        #[from]
        error: fedora::OpenIDClientError,
    },
}

impl<'a> BodhiServiceBuilder<'a> {
    // This method creates a new builder for the "production" instances of the fedora services.
    pub fn default() -> Self {
        BodhiServiceBuilder {
            service_type: BodhiServiceType::DEFAULT,
            authentication: None,
            url: FEDORA_BODHI_URL.to_string(),
            timeout: None,
            retries: None,
        }
    }

    // This method creates a new builder for the "staging" instances of the fedora services.
    pub fn staging() -> Self {
        BodhiServiceBuilder {
            service_type: BodhiServiceType::STAGING,
            authentication: None,
            url: FEDORA_BODHI_STG_URL.to_string(),
            timeout: None,
            retries: None,
        }
    }

    // This method creates a custom builder, where both bodhi URL and authentication endpoint need
    // to be specified manually.
    pub fn custom(url: String, openid_url: String) -> Self {
        BodhiServiceBuilder {
            service_type: BodhiServiceType::CUSTOM { openid_url },
            authentication: None,
            url,
            timeout: None,
            retries: None,
        }
    }

    // This method can be used to override the default request timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    // This method can be used to override the default number of retries.
    #[must_use]
    pub fn retries(mut self, retries: usize) -> Self {
        self.retries = Some(retries);
        self
    }

    // This method can be used to set credentials for authenticating with the fedora OpenID
    // endpoint, so the resulting [`BodhiService`](struct.BodhiService.html) can be used to
    // send authenticated requests for creating and editing things on the server.
    #[must_use]
    pub fn authentication(mut self, username: &'a str, password: &'a str) -> Self {
        self.authentication = Some(Authentication { username, password });
        self
    }

    // This method builds a [`BodhiService`](struct.BodhiService.html) given the arguments that
    // were supplied to this [`BodhiServiceBuilder`](struct.BodhiServiceBuilder.html), including
    // an attempt to authenticate with the fedora OpenID endpoint if credentials have been
    // supplied.
    pub async fn build(self) -> Result<BodhiService, BuilderError> {
        let url = Url::parse(&self.url)?;

        let timeout = match self.timeout {
            Some(timeout) => timeout,
            None => REQUEST_TIMEOUT,
        };

        let retries = match self.retries {
            Some(retries) => retries,
            None => REQUEST_RETRIES,
        };

        let login_url = url.join("/login")?;
        let user_agent = USER_AGENT.to_string();

        let session = if let Some(auth) = self.authentication {
            match self.service_type {
                BodhiServiceType::DEFAULT => {
                    Session::openid_auth(login_url, OpenIDSessionKind::Default)
                        .user_agent(&user_agent)
                        .timeout(timeout)
                        .build()
                        .login(auth.username, auth.password)
                        .await?
                },
                BodhiServiceType::STAGING => {
                    Session::openid_auth(login_url, OpenIDSessionKind::Staging)
                        .user_agent(&user_agent)
                        .timeout(timeout)
                        .build()
                        .login(auth.username, auth.password)
                        .await?
                },
                BodhiServiceType::CUSTOM { openid_url } => {
                    let auth_url = Url::parse(&openid_url)?;

                    Session::openid_auth(login_url, OpenIDSessionKind::Custom { auth_url })
                        .user_agent(&user_agent)
                        .timeout(timeout)
                        .build()
                        .login(auth.username, auth.password)
                        .await?
                },
            }
        } else {
            Session::anonymous().user_agent(&user_agent).timeout(timeout).build()
        };

        Ok(BodhiService { url, session, retries })
    }
}

// This struct represents a specific bodhi service, typically running remotely, although a local
// URL could be specified, as well. This BodhiService instance is then used by queries to actually
// submit to, and receive from - the service.
#[derive(Debug)]
pub struct BodhiService {
    url: Url,
    session: Session,
    retries: usize,
}

async fn try_get(session: &Client, url: Url, body: Option<String>) -> Result<Response, ServiceError> {
    let response = match body {
        Some(body) => session.get(url).body(body).send().await,
        None => session.get(url).send().await,
    };

    match response {
        Ok(response) => {
            match response.content_length() {
                Some(_len) => {
                    // return the first valid response
                    Ok(response)
                },
                None => {
                    // response is empty
                    log::warn!("Invalid server response: Expected JSON but received empty body.");
                    Err(ServiceError::EmptyResponseError)
                },
            }
        },
        Err(error) => {
            // take a breath, and keep on trying (or not)
            Err(ServiceError::RequestError { error })
        },
    }
}

async fn retry_get(session: &Client, url: Url, body: Option<String>, retries: usize) -> Result<Response, ServiceError> {
    let mut retries: Vec<Duration> = vec![Duration::from_secs(1); retries];

    let result = loop {
        if let Some(duration) = retries.pop() {
            match try_get(session, url.clone(), body.clone()).await {
                Ok(result) => break Ok(result),
                Err(error) => {
                    log::warn!("Retrying failed HTTP request: {}", error);
                    // FIXME: this will block the async runtime
                    std::thread::sleep(duration);
                },
            }
        } else {
            match try_get(session, url, body).await {
                Ok(result) => break Ok(result),
                Err(error) => break Err(error),
            }
        }
    };

    result
}

async fn try_post(session: &Client, url: Url, body: Option<String>) -> Result<Response, ServiceError> {
    let response = match body {
        Some(body) => session.post(url).body(body).send().await,
        None => session.post(url).send().await,
    };

    match response {
        Ok(response) => {
            match response.content_length() {
                Some(_len) => {
                    // return the first valid response
                    Ok(response)
                },
                None => {
                    // response is empty
                    log::warn!("Invalid server response: Expected JSON but received empty body.");
                    Err(ServiceError::EmptyResponseError)
                },
            }
        },
        Err(error) => {
            // take a breath, and keep on trying (or not)
            Err(ServiceError::RequestError { error })
        },
    }
}

impl BodhiService {
    fn session(&self) -> &Client {
        self.session.session()
    }
}

impl BodhiService {
    pub async fn request<P, T>(&self, request: &dyn SingleRequest<P, T>) -> Result<T, QueryError>
    where
        T: DeserializeOwned,
    {
        match request.method() {
            RequestMethod::GET => self.request_get(request).await,
            RequestMethod::POST => self.request_post(request).await,
        }
    }

    async fn request_get<P, T>(&self, request: &dyn SingleRequest<P, T>) -> Result<T, QueryError>
    where
        T: DeserializeOwned,
    {
        let page = self.page_request_get(request).await?;
        Ok(request.extract(page))
    }

    async fn page_request_get<P, T>(&self, request: &dyn SingleRequest<P, T>) -> Result<P, QueryError>
    where
        T: DeserializeOwned,
    {
        let url = self
            .url
            .join(&request.path()?)
            .map_err(|e| ServiceError::UrlParsingError { error: e })?;
        let response = retry_get(self.session(), url, request.body(None)?, self.retries).await?;
        let status = response.status();

        let page = if status.is_success() {
            let string = response.text().await?;
            let page = request.parse(&string)?;
            Ok(page)
        } else if status == 404 {
            Err(QueryError::NotFound)
        } else {
            let result = response.text().await?;
            let error: BodhiError = serde_json::from_str(&result)?;

            Err(QueryError::BodhiError { error })
        };

        page
    }

    async fn request_post<P, T>(&self, request: &dyn SingleRequest<P, T>) -> Result<T, QueryError>
    where
        T: DeserializeOwned,
    {
        let page = self.page_request_post(request).await?;
        Ok(request.extract(page))
    }

    async fn page_request_post<P, T>(&self, request: &dyn SingleRequest<P, T>) -> Result<P, QueryError>
    where
        T: DeserializeOwned,
    {
        let token = self.request_get(&CSRFQuery::new()).await?;
        let url = self
            .url
            .join(&request.path()?)
            .map_err(|e| ServiceError::UrlParsingError { error: e })?;
        let response = try_post(self.session(), url, request.body(Some(token))?).await?;
        let status = response.status();

        let page = if status.is_success() {
            let string = response.text().await?;
            let page = request.parse(&string)?;
            Ok(page)
        } else if status == 404 {
            Err(QueryError::NotFound)
        } else {
            let result = response.text().await?;
            let error: BodhiError = serde_json::from_str(&result)?;

            Err(QueryError::BodhiError { error })
        };

        page
    }

    pub async fn paginated_request<P, V, T>(&self, request: &dyn PaginatedRequest<P, V>) -> Result<Vec<T>, QueryError>
    where
        P: Pagination,
        V: IntoIterator<Item = T> + DeserializeOwned,
        T: DeserializeOwned,
    {
        let mut results: Vec<T> = Vec::new();
        request.callback(0, 1);

        let first_request = request.page_request(1);

        let first_page = self.page_request_get(first_request.as_ref()).await?;

        let mut page = 2u32;
        let mut pages = first_page.pages();

        request.callback(1, pages);

        results.extend(first_request.extract(first_page));

        while page <= pages {
            let page_request = request.page_request(page);

            let next_page = self.page_request_get(page_request.as_ref()).await?;
            request.callback(page, pages);

            page += 1;
            pages = next_page.pages();

            results.extend(page_request.extract(next_page).into_iter());
        }

        Ok(results)
    }
}
