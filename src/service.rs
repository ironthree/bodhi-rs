//! This module contains the structures and methods to interact with a (remote)
//! bodhi server instance.

use std::collections::HashMap;
use std::time::Duration;

use failure::Fail;
use fedora::{OpenIDClient, OpenIDClientBuilder};
use reqwest::{Response, Url};

use crate::{FEDORA_BODHI_URL, FEDORA_BODHI_STG_URL};

/// Always start with page 1 for multi-page queries.
/// Everything else would be stupid.
pub const DEFAULT_PAGE: u32 = 1;

/// This constant defines how many items are queried every time for multi-page queries.
/// The maximum is 100, the default is 20, and 50 seems a good compromise for speed.
pub const DEFAULT_ROWS: u32 = 50;

/// Specify a longer timeout duration (60 s) for bodhi requests.
/// The `reqwest` default value of 30 seconds is a bit too short for long-running queries.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Specify a number of retries in case of connection failures.
const REQUEST_RETRIES: usize = 3;

#[derive(Debug)]
enum BodhiServiceType {
    DEFAULT,
    STAGING,
    // CUSTOM,
}

// TODO
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::default()
///     .timeout(std::time::Duration::from_secs(42))
///     .retries(9001);
/// ```
#[derive(Debug)]
pub struct BodhiServiceBuilder {
    service_type: BodhiServiceType,
    url: String,
    timeout: Option<Duration>,
    retries: Option<usize>,
}

#[derive(Debug, Fail)]
pub enum BuilderError {
    #[fail(display = "Failed to parse service URL: {}", error)]
    UrlParsingError { error: reqwest::UrlError },
    #[fail(display = "Failed to initialize OpenID client: {}", error)]
    OpenIDClientError { error: fedora::openid::BuilderError },
}

impl From<reqwest::UrlError> for BuilderError {
    fn from(error: reqwest::UrlError) -> Self {
        BuilderError::UrlParsingError { error }
    }
}

impl From<fedora::openid::BuilderError> for BuilderError {
    fn from(error: fedora::openid::BuilderError) -> Self {
        BuilderError::OpenIDClientError { error }
    }
}

impl BodhiServiceBuilder {
    // TODO
    pub fn default() -> Self {
        BodhiServiceBuilder {
            service_type: BodhiServiceType::DEFAULT,
            url: FEDORA_BODHI_URL.to_string(),
            timeout: None,
            retries: None,
        }
    }

    // TODO
    pub fn staging() -> Self {
        BodhiServiceBuilder {
            service_type: BodhiServiceType::STAGING,
            url: FEDORA_BODHI_STG_URL.to_string(),
            timeout: None,
            retries: None,
        }
    }

    /*
    // TODO
    pub fn custom(url: String, openid_url: String) -> Self {
        BodhiServiceBuilder {
            service_type: BodhiServiceType::CUSTOM,
            url,
            timeout: None,
            retries: None,
        }
    }
    */

    // TODO
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    // TODO
    pub fn retries(mut self, retries: usize) -> Self {
        self.retries = Some(retries);
        self
    }

    // TODO
    pub fn build(self) -> Result<BodhiService, BuilderError> {
        let url = Url::parse(&self.url)?;

        let timeout = match self.timeout {
            Some(timeout) => timeout,
            None => REQUEST_TIMEOUT,
        };

        let retries = match self.retries {
            Some(retries) => retries,
            None => REQUEST_RETRIES,
        };

        let session = match self.service_type {
            BodhiServiceType::DEFAULT => {
                OpenIDClientBuilder::default()
                    .user_agent(String::from("bodhi-rs"))
                    .timeout(timeout)
                    .build()?
            },
            BodhiServiceType::STAGING => {
                OpenIDClientBuilder::staging()
                    .user_agent(String::from("bodhi-rs"))
                    .timeout(timeout)
                    .build()?
            },
            // BodhiServiceType::CUSTOM
        };

        Ok(BodhiService {
            url,
            session,
            username: None,
            authenticated: false,
            retries,
        })
    }
}

/// This struct represents a specific bodhi service, typically running remotely,
/// although a local URL could be specified, as well. This BodhiService instance
/// is then used by queries to actually submit to, and receive from - the service.
#[derive(Debug)]
pub struct BodhiService {
    url: Url,
    session: OpenIDClient,
    username: Option<String>,
    authenticated: bool,
    retries: usize,
}

#[derive(Debug, Fail)]
pub enum ServiceError {
    #[fail(display = "Failed to authenticate with OpenID provider: {}", error)]
    AuthenticationError { error: fedora::openid::ClientError },
    #[fail(display = "Authorization required but not provided.")]
    NotAuthenticated,
    #[fail(display = "Failed to query bodhi instance: {}", error)]
    RequestError { error: reqwest::Error },
    #[fail(display = "Failed to parse redirection URL: {}", error)]
    UrlParsingError { error: reqwest::UrlError },
    #[fail(display = "Received an empty response.")]
    EmptyResponseError,
    #[fail(display = "Retrying a failed request failed repeatedly.")]
    RetryError,
}

impl From<reqwest::Error> for ServiceError {
    fn from(error: reqwest::Error) -> Self {
        ServiceError::RequestError { error }
    }
}

impl From<reqwest::UrlError> for ServiceError {
    fn from(error: reqwest::UrlError) -> Self {
        ServiceError::UrlParsingError { error }
    }
}

impl BodhiService {
    // TODO
    pub fn authenticate(&mut self, username: String, password: String) -> Result<(), ServiceError> {
        match self.session.login(username.clone(), password) {
            Ok(_) => {
                self.username = Some(username);
                self.authenticated = true;
                Ok(())
            }
            Err(error) => Err(ServiceError::AuthenticationError { error }),
        }
    }

    // TODO
    pub fn authenticated(&self) -> bool {
        self.authenticated
    }

    // TODO
    pub fn username(&self) -> Result<String, ServiceError> {
        match &self.username {
            Some(username) => Ok(username.clone()),
            None => Err(ServiceError::NotAuthenticated),
        }
    }

    // TODO
    pub(crate) fn get(
        &self,
        path: &str,
        args: Option<HashMap<&str, String>>,
    ) -> Result<Response, ServiceError> {
        let url = self.url.join(path)?;

        let query: Vec<(&str, String)> = match args {
            Some(mut map) => map.drain().collect(),
            None => Vec::new(),
        };

        let qf = || {
            match self.session.session().get(url.clone()).query(&query).send() {
                Ok(response) => {
                    match response.content_length() {
                        Some(_len) => {
                            // return the first valid response
                            Ok(response)
                        }
                        None => {
                            // response is empty
                            Err(ServiceError::EmptyResponseError)
                        }
                    }
                }

                Err(error) => {
                    // take a breath, and keep on trying (or not)
                    Err(ServiceError::RequestError { error })
                }
            }
        };

        let retries: Vec<Duration> = vec![Duration::from_secs(1); REQUEST_RETRIES];
        match retry::retry(retries, qf) {
            Ok(response) => Ok(response),
            Err(error) => {
                if let retry::Error::Operation { error: inner, .. } = error {
                    Err(inner)
                } else {
                    Err(ServiceError::RetryError)
                }
            }
        }
    }

    // TODO
    pub(crate) fn post(
        &self,
        path: &str,
        body: String,
        args: Option<HashMap<&str, String>>,
    ) -> Result<Response, ServiceError> {
        let url = self.url.join(path)?;

        let query: Vec<(&str, String)> = match args {
            Some(mut map) => map.drain().collect(),
            None => Vec::new(),
        };

        let response = self
            .session
            .session()
            .post(url)
            .body(body)
            .query(&query)
            .send()?;

        Ok(response)
    }
}
