//! This module contains the structures and methods to interact with a (remote) bodhi server
//! instance.

use std::fmt::{Debug, Formatter};
use std::time::Duration;

use failure::Fail;
use fedora::{AnonymousSessionBuilder, OpenIDSessionBuilder, Session};
use reqwest::blocking::Response;
use url::Url;

use crate::data::{FEDORA_BODHI_STG_URL, FEDORA_BODHI_URL};
use crate::error::{QueryError, ServiceError};
use crate::{Create, Edit, Query};

/// This constant defines how many items are queried every time for multi-page queries. The
/// server-side maximum is usually 100, the default is 20, and 50 seems a good compromise for speed.
pub const DEFAULT_ROWS: u32 = 50;

/// Specify a longer timeout duration (60 s) for bodhi requests. The `reqwest` default value of 30
/// seconds is a bit too short for long-running queries.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Specify a number of retries in case of connection or transient server failures.
const REQUEST_RETRIES: usize = 3;

/// Specify a sane default user agent for bodhi-rs.
const USER_AGENT: &str = "bodhi-rs";

#[derive(Debug)]
enum BodhiServiceType {
    DEFAULT,
    STAGING,
    CUSTOM { openid_url: String },
}

/// This struct contains information necessary to build an instance of
/// [`BodhiService`](struct.BodhiService.html) with the necessary flags. Additionally, depending on
/// whether username and password were supplied as arguments, building the service instance will
/// try to return a privileged session by authenticating via the fedora OpenID endpoint first.
///
/// It's possible to target either the fedora production or the staging instances of bodhi, or
/// provide a custom URL, via the `default()`, `staging()`, and `custom()` methods, respectively.
///
/// ```
/// // create service with anonymous session
/// let bodhi = bodhi::BodhiServiceBuilder::default()
///     .timeout(std::time::Duration::from_secs(42))
///     .retries(9001)
///     .build();
/// ```
///
/// ```
/// // builder for an authenticated session
/// let builder = bodhi::BodhiServiceBuilder::staging()
///     .timeout(std::time::Duration::from_secs(120))
///     .retries(2)
///     .authentication("bodhi-rs", "password1");
/// // builder.build();
/// ```
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

/// This enum contains variants for all the ways in which constructing a
/// [`BodhiService`](struct.BodhiService.html) instance can fail.
#[derive(Debug, Fail)]
pub enum BuilderError {
    /// This error represents an issue while parsing user-supplied URLs. If should never be returned
    /// for the default settings, since the hardcoded URLs should always be valid.
    #[fail(display = "Failed to parse service URL: {}", error)]
    UrlParsingError {
        /// The inner error contains the issue that occurred while parsing the invalid URL.
        error: url::ParseError,
    },
    /// This error represents an issue that occurred during authentication via the OpenID API.
    #[fail(display = "Failed to initialize OpenID client: {}", error)]
    OpenIDClientError {
        /// The inner error contains the issue that occurred during the set up of an authenticated
        /// session via an OpenID endpoint.
        error: fedora::openid::OpenIDClientError,
    },
    /// This error represents an HTTP client library initialisation error.
    #[fail(display = "Failed to initialize the HTTP client: {}", error)]
    InitialisationError {
        /// The inner error contains the issue that occurred during initialisation of the HTTP
        /// client library.
        error: fedora::anonymous::InitialisationError,
    },
}

impl From<url::ParseError> for BuilderError {
    fn from(error: url::ParseError) -> Self {
        BuilderError::UrlParsingError { error }
    }
}

impl From<fedora::openid::OpenIDClientError> for BuilderError {
    fn from(error: fedora::openid::OpenIDClientError) -> Self {
        BuilderError::OpenIDClientError { error }
    }
}

impl From<fedora::anonymous::InitialisationError> for BuilderError {
    fn from(error: fedora::anonymous::InitialisationError) -> Self {
        BuilderError::InitialisationError { error }
    }
}

impl<'a> BodhiServiceBuilder<'a> {
    /// This method creates a new builder for the "production" instances of the fedora services.
    pub fn default() -> Self {
        BodhiServiceBuilder {
            service_type: BodhiServiceType::DEFAULT,
            authentication: None,
            url: FEDORA_BODHI_URL.to_string(),
            timeout: None,
            retries: None,
        }
    }

    /// This method creates a new builder for the "staging" instances of the fedora services.
    pub fn staging() -> Self {
        BodhiServiceBuilder {
            service_type: BodhiServiceType::STAGING,
            authentication: None,
            url: FEDORA_BODHI_STG_URL.to_string(),
            timeout: None,
            retries: None,
        }
    }

    /// This method creates a custom builder, where both bodhi URL and authentication endpoint need
    /// to be specified manually.
    pub fn custom(url: String, openid_url: String) -> Self {
        BodhiServiceBuilder {
            service_type: BodhiServiceType::CUSTOM { openid_url },
            authentication: None,
            url,
            timeout: None,
            retries: None,
        }
    }

    /// This method can be used to override the default request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// This method can be used to override the default number of retries.
    pub fn retries(mut self, retries: usize) -> Self {
        self.retries = Some(retries);
        self
    }

    /// This method can be used to set credentials for authenticating with the fedora OpenID
    /// endpoint, so the resulting [`BodhiService`](struct.BodhiService.html) can be used to
    /// send authenticated requests for creating and editing things on the server.
    pub fn authentication(mut self, username: &'a str, password: &'a str) -> Self {
        self.authentication = Some(Authentication { username, password });
        self
    }

    /// This method builds a [`BodhiService`](struct.BodhiService.html) given the arguments that
    /// were supplied to this [`BodhiServiceBuilder`](struct.BodhiServiceBuilder.html), including
    /// an attempt to authenticate with the fedora OpenID endpoint if credentials have been
    /// supplied.
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

        let login_url = url.join("/login")?;
        let user_agent = USER_AGENT.to_string();

        let session: Box<dyn Session> = if let Some(auth) = self.authentication {
            match self.service_type {
                BodhiServiceType::DEFAULT => Box::new(
                    OpenIDSessionBuilder::default(login_url, auth.username, auth.password)
                        .user_agent(&user_agent)
                        .timeout(timeout)
                        .build()?,
                ),
                BodhiServiceType::STAGING => Box::new(
                    OpenIDSessionBuilder::staging(login_url, auth.username, auth.password)
                        .user_agent(&user_agent)
                        .timeout(timeout)
                        .build()?,
                ),
                BodhiServiceType::CUSTOM { openid_url } => {
                    let url = Url::parse(&openid_url)?;

                    Box::new(
                        OpenIDSessionBuilder::custom(url, login_url, auth.username, auth.password)
                            .user_agent(&user_agent)
                            .timeout(timeout)
                            .build()?,
                    )
                },
            }
        } else {
            Box::new(
                AnonymousSessionBuilder::new()
                    .user_agent(&user_agent)
                    .timeout(timeout)
                    .build()?,
            )
        };

        Ok(BodhiService { url, session, retries })
    }
}

/// This struct represents a specific bodhi service, typically running remotely, although a local
/// URL could be specified, as well. This BodhiService instance is then used by queries to actually
/// submit to, and receive from - the service.
pub struct BodhiService {
    url: Url,
    session: Box<dyn Session>,
    retries: usize,
}

impl Debug for BodhiService {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "BodhiService {{ url: {}, retries: {} }}", &self.url, self.retries)
    }
}

impl BodhiService {
    pub(crate) fn get(&self, path: &str) -> Result<Response, ServiceError> {
        let url = self.url.join(path)?;

        let qf = || {
            match self.session.session().get(url.clone()).send() {
                Ok(response) => {
                    match response.content_length() {
                        Some(_len) => {
                            // return the first valid response
                            Ok(response)
                        },
                        None => {
                            // response is empty
                            Err(ServiceError::EmptyResponseError)
                        },
                    }
                },
                Err(error) => {
                    // take a breath, and keep on trying (or not)
                    Err(ServiceError::RequestError { error })
                },
            }
        };

        let retries: Vec<Duration> = vec![Duration::from_secs(1); self.retries];
        match retry::retry(retries, qf) {
            Ok(response) => {
                #[cfg(feature = "debug")]
                {
                    dbg!(&response);
                };

                Ok(response)
            },
            Err(error) => {
                if let retry::Error::Operation { error: inner, .. } = error {
                    Err(inner)
                } else {
                    Err(ServiceError::RetryError)
                }
            },
        }
    }

    pub(crate) fn post(&self, path: &str, body: String) -> Result<Response, ServiceError> {
        let url = self.url.join(path)?;

        let response = self.session.session().post(url).body(body).send()?;

        #[cfg(feature = "debug")]
        {
            dbg!(&response);
        }

        Ok(response)
    }

    /// This method is used for GET methods to query things on the bodhi instance.
    pub fn query<T>(&self, query: &dyn Query<T>) -> Result<T, QueryError> {
        Query::query(query, self)
    }

    /// This method is used for POST methods to create new things on the bodhi instance.
    pub fn create<T>(&self, creator: &dyn Create<T>) -> Result<T, QueryError> {
        Create::create(creator, self)
    }

    /// This method is used for POST methods to edit existing things on the bodhi instance.
    pub fn edit<T>(&self, editor: &dyn Edit<T>) -> Result<T, QueryError> {
        Edit::edit(editor, self)
    }
}
