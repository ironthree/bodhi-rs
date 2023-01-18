//! # bodhi API client implementation
//!
//! This module contains data structures and implementations for creating a bodhi client session,
//! and for sending requests to a bodhi server.

use std::time::Duration;

use fedora::reqwest::{Client, Response};
use fedora::url::{self, Url};
use fedora::{OpenIDSessionKind, Session};
use serde::de::DeserializeOwned;

use crate::data::{FEDORA_BODHI_STG_URL, FEDORA_BODHI_URL};
use crate::error::{BodhiError, QueryError};
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};
use crate::CSRFQuery;

// This constant defines how many items are queried every time for multi-page queries. The
// server-side maximum is 100, the default is 20, and 50 seems to be a good compromise between
// the frequency of server timeouts, request failures, and query speed.
pub(crate) const DEFAULT_ROWS: u32 = 50;

// Specify a longer timeout duration (60 s) for bodhi requests. The `reqwest` default value of 30
// seconds is a bit too short for long-running queries.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

// Specify a number of retries in case of connection or transient server failures.
const REQUEST_RETRIES: usize = 3;

// Specify a sane default user agent for bodhi-rs.
const USER_AGENT: &str = concat!("bodhi-rs v", env!("CARGO_PKG_VERSION"));


#[derive(Debug)]
enum BodhiServiceType {
    Default,
    Staging,
    Custom { openid_url: String },
}


/// This data type contains all information that is required to build a [`BodhiClient`] instance
/// with necessary flags. Additionally, depending on whether username and password are supplied as
/// arguments, building the service instance will try to return a privileged session by
/// authenticating with the specified OpenID endpoint first.
///
/// It's possible to target either the Fedora production or the staging instances of bodhi, or
/// provide a custom URL, via the `default()`, `staging()`, and `custom()` methods, respectively.
///
/// ```
/// // create service with anonymous session
/// let bodhi = bodhi::BodhiClientBuilder::default()
///     .timeout(std::time::Duration::from_secs(42))
///     .retries(9001)
///     .build();
/// ```
///
/// ```no_run
/// // builder for an authenticated session
/// let builder = bodhi::BodhiClientBuilder::staging()
///     .timeout(std::time::Duration::from_secs(120))
///     .retries(2)
///     .authentication("bodhi-rs", "password1");
/// let bodhi = builder.build();
/// ```
#[derive(Debug)]
pub struct BodhiClientBuilder<'a> {
    service_type: BodhiServiceType,
    authentication: Option<Authentication<'a>>,
    url: String,
    timeout: Option<Duration>,
    user_agent: Option<&'a str>,
    retries: Option<usize>,
}

#[derive(Debug)]
struct Authentication<'a> {
    username: &'a str,
    password: &'a str,
}


/// error type that represents a failure that occurs while initializing a [`BodhiClient`]
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    /// error while parsing base URL or login URL
    ///
    /// This error should only ever be returned when using custom URLs, never when constructing a
    /// client for the default Fedora (production) or staging instances of bodhi.
    #[error("Failed to parse service URL: {error}")]
    UrlParsingError {
        /// error that occurred while parsing the URL
        #[from]
        error: url::ParseError,
    },
    /// error while authenticating with an OpenID endpoint
    #[error("Failed to initialize OpenID client: {error}")]
    OpenIDClientError {
        /// error that occurred during the OpenID authentication process
        #[from]
        error: fedora::OpenIDClientError,
    },
}

impl<'a> BodhiClientBuilder<'a> {
    #[allow(clippy::should_implement_trait)]
    /// constructor for [`BodhiClientBuilder`] for the default / production instance of bodhi
    pub fn default() -> Self {
        BodhiClientBuilder {
            service_type: BodhiServiceType::Default,
            authentication: None,
            url: FEDORA_BODHI_URL.to_string(),
            timeout: None,
            user_agent: None,
            retries: None,
        }
    }

    /// constructor for [`BodhiClientBuilder`] for the staging instance of bodhi
    pub fn staging() -> Self {
        BodhiClientBuilder {
            service_type: BodhiServiceType::Staging,
            authentication: None,
            url: FEDORA_BODHI_STG_URL.to_string(),
            timeout: None,
            user_agent: None,
            retries: None,
        }
    }

    /// constructor for [`BodhiClientBuilder`] with custom settings (user-specified base URLs)
    pub fn custom(url: String, openid_url: String) -> Self {
        BodhiClientBuilder {
            service_type: BodhiServiceType::Custom { openid_url },
            authentication: None,
            url,
            timeout: None,
            user_agent: None,
            retries: None,
        }
    }

    /// method for overriding the default network request timeout
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// method for overriding the default User-Agent HTTP header that is used for requests
    #[must_use]
    pub fn user_agent(mut self, user_agent: &'a str) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    /// method for overriding the default number of retry attempts for read-only requests
    #[must_use]
    pub fn retries(mut self, retries: usize) -> Self {
        self.retries = Some(retries);
        self
    }

    /// method for supplying username and password when using an authenticated bodhi API client
    #[must_use]
    pub fn authentication(mut self, username: &'a str, password: &'a str) -> Self {
        self.authentication = Some(Authentication { username, password });
        self
    }

    /// method for building a [`BodhiClient`] based on the parameters in this [`BodhiClientBuilder`]
    ///
    /// If authentication parameters (username and password) have been supplied as arguments as
    /// well, calling this method will also attempt to authenticate via OpenID.
    pub async fn build(self) -> Result<BodhiClient, BuilderError> {
        let url = Url::parse(&self.url)?;
        let login_url = url.join("/login?method=openid")?;

        let timeout = self.timeout.unwrap_or(REQUEST_TIMEOUT);
        let retries = self.retries.unwrap_or(REQUEST_RETRIES);
        let user_agent = self.user_agent.unwrap_or(USER_AGENT).to_string();

        let session = if let Some(auth) = self.authentication {
            match self.service_type {
                BodhiServiceType::Default => {
                    Session::openid_auth(login_url, OpenIDSessionKind::Default)
                        .user_agent(&user_agent)
                        .timeout(timeout)
                        .build()
                        .login(auth.username, auth.password)
                        .await?
                },
                BodhiServiceType::Staging => {
                    Session::openid_auth(login_url, OpenIDSessionKind::Staging)
                        .user_agent(&user_agent)
                        .timeout(timeout)
                        .build()
                        .login(auth.username, auth.password)
                        .await?
                },
                BodhiServiceType::Custom { openid_url } => {
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

        Ok(BodhiClient { url, session, retries })
    }
}


/// data type that encapsulates all information that is required for making network requests
///
/// A successfully constructed [`BodhiClient`] contains a valid base URL for the given bodhi server
/// instance, and a networking session that is set up with all necessary headers and cookies.
#[derive(Debug)]
pub struct BodhiClient {
    url: Url,
    session: Session,
    retries: usize,
}

async fn try_get(session: &Client, url: Url, body: Option<String>) -> Result<Response, QueryError> {
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
                    Err(QueryError::EmptyResponse)
                },
            }
        },
        Err(error) => {
            // take a breath, and keep on trying (or not)
            Err(QueryError::RequestError { error })
        },
    }
}

async fn retry_get(session: &Client, url: Url, body: Option<String>, retries: usize) -> Result<Response, QueryError> {
    let mut retries: Vec<Duration> = vec![Duration::from_secs(1); retries];

    loop {
        if let Some(duration) = retries.pop() {
            match try_get(session, url.clone(), body.clone()).await {
                Ok(result) => break Ok(result),
                Err(error) => {
                    log::warn!("Retrying failed HTTP request: {}", error);
                    tokio::time::sleep(duration).await;
                },
            }
        } else {
            match try_get(session, url, body).await {
                Ok(result) => break Ok(result),
                Err(error) => break Err(error),
            }
        }
    }
}

async fn try_post(session: &Client, url: Url, body: Option<String>) -> Result<Response, QueryError> {
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
                    Err(QueryError::EmptyResponse)
                },
            }
        },
        Err(error) => {
            // take a breath, and keep on trying (or not)
            Err(QueryError::RequestError { error })
        },
    }
}

async fn handle_response<P, T>(response: Response, request: &dyn SingleRequest<P, T>) -> Result<P, QueryError>
where
    T: DeserializeOwned,
{
    let status = response.status();

    if status.is_success() {
        let string = response.text().await?;
        let page = request.parse(&string)?;
        Ok(page)
    } else if status == 404 {
        Err(QueryError::NotFound)
    } else {
        let result = response.text().await?;
        let error: BodhiError = serde_json::from_str(&result)?;
        Err(QueryError::BodhiError { error })
    }
}

impl BodhiClient {
    fn session(&self) -> &Client {
        self.session.session()
    }

    /// async method for making a single-page `GET` or a `POST` request
    ///
    /// This method is used to handle single-page `GET` and `POST` requests. By default, `GET`
    /// requests are retried for the specified number of times (default: 3) before an error is
    /// returned. `POST` requests are not retried, because they might have already modified server
    /// state even if the request timed out or returned an error.
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
            .map_err(|e| QueryError::UrlParsingError { error: e })?;
        let response = retry_get(self.session(), url, request.body(None)?, self.retries).await?;

        handle_response(response, request).await
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
            .map_err(|e| QueryError::UrlParsingError { error: e })?;
        let response = try_post(self.session(), url, request.body(Some(token))?).await?;

        handle_response(response, request).await
    }

    /// async method for making multi-page / paginated `GET` requests
    ///
    /// This method is used to handle paginated `GET` requests. Internally, this will result in a
    /// stream of single-page requests to be handled by [`BodhiClient::request`]. This method
    /// is intended to be more convenient than manually constructing and executing single-page
    /// requests, handling errors, and then reassembling the results - as those things are all
    /// handled by this method internally.
    pub async fn paginated_request<P, V, T>(&self, request: &dyn PaginatedRequest<P, V>) -> Result<Vec<T>, QueryError>
    where
        P: Pagination,
        V: IntoIterator<Item = T> + DeserializeOwned,
        T: DeserializeOwned,
    {
        let mut results: Vec<T> = Vec::new();

        // initialize progress callback with "zero progress"
        request.callback(0, 1);

        let first_request = request.page_request(1);
        let first_page = self.page_request_get(first_request.as_ref()).await?;

        let mut page = 2u32;
        let mut pages = first_page.pages();

        // update progress callback with actual total pages
        request.callback(1, pages);

        results.extend(first_request.extract(first_page));

        while page <= pages {
            let page_request = request.page_request(page);
            let next_page = self.page_request_get(page_request.as_ref()).await?;

            request.callback(page, pages);

            page += 1;
            pages = next_page.pages();

            results.extend(page_request.extract(next_page));
        }

        Ok(results)
    }
}
