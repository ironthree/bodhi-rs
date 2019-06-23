//! This module contains the structures and methods to interact with a (remote)
//! bodhi server instance.

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use failure::Fail;
use fedora::{OpenIDClient, OpenIDClientBuilder};
use reqwest::{Response, Url};

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
const REQUEST_RETRIES: u32 = 3;

/// TODO
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::new(String::from("https://bodhi.fedoraproject.org"))
///     .timeout(std::time::Duration::from_secs(42))
///     .retries(9001);
/// ```
#[derive(Debug)]
pub struct BodhiServiceBuilder {
    url: String,
    timeout: Option<Duration>,
    retries: Option<u32>,
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
    /// TODO
    pub fn new(url: String) -> Self {
        BodhiServiceBuilder {
            url,
            timeout: None,
            retries: None,
        }
    }

    /// TODO
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// TODO
    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    /// TODO
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

        let session = OpenIDClientBuilder::new(url.clone())
            .user_agent(String::from("bodhi-rs"))
            .timeout(timeout)
            .build()?;

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
    retries: u32,
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
    /// TODO
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

    /// TODO
    pub fn authenticated(&self) -> bool {
        self.authenticated
    }

    /// TODO
    pub fn username(&self) -> Result<String, ServiceError> {
        match &self.username {
            Some(username) => Ok(username.clone()),
            None => Err(ServiceError::NotAuthenticated),
        }
    }

    /// TODO
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

        // retry in case of connection failures
        let mut retries = REQUEST_RETRIES;
        let mut errors: Vec<ServiceError> = Vec::new();

        while retries > 0 {
            match self.session.session().get(url.clone()).query(&query).send() {
                Ok(response) => {
                    match response.content_length() {
                        None => {
                            // response is empty
                            errors.push(ServiceError::EmptyResponseError);
                            sleep(Duration::from_secs(1));
                            retries -= 1;
                        }
                        Some(_len) => {
                            // return the first valid response
                            return Ok(response);
                        }
                    }
                }

                Err(error) => {
                    // take a breath, and keep on trying (or not)
                    errors.push(ServiceError::RequestError { error });
                    sleep(Duration::from_secs(1));
                    retries -= 1;
                }
            }
        }

        // fail if the connection keeps failing
        Err(errors.into_iter().last().unwrap())
    }

    /*
    /// TODO
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
    */
}
