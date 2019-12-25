//! This module contains some common error types for wrapping networking-related issues,
//! server-side issues, and client-side issues (including JSON deserialization problems).
use std::collections::HashMap;

use failure::Fail;
use serde::Deserialize;

/// This struct contains error messages that are deserialized from bodhi's error responses.
#[derive(Debug, Deserialize, Fail)]
pub struct BodhiError {
    /// This field contains a list of server-side error messages.
    pub errors: Vec<HashMap<String, String>>,
    /// This field contains the server-side status message for the failure.
    pub status: String,
}

impl std::fmt::Display for BodhiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:#?}", self)
    }
}

/// This enum encapsulates the different ways in which bodhi queries can fail.
#[derive(Debug, Fail)]
pub enum QueryError {
    /// This error represents a network-related issue that occurred within
    /// [`reqwest`](https://docs.rs/reqwest).
    #[fail(display = "Failed to query bodhi service: {}", error)]
    RequestError {
        /// The inner error contains the error passed from [`reqwest`](https://docs.rs/reqwest).
        error: reqwest::Error,
    },
    /// This error represents an issue with deserializing JSON request data. If this ever happens,
    /// it is almost certainly a bug in this crate.
    #[fail(display = "Failed to deserialize JSON response: {}", error)]
    DeserializationError {
        /// The inner error contains the deserialization error message from
        /// [`serde_json`](https://docs.rs/serde_json).
        error: serde_json::Error,
    },
    /// This error represents an issue with serializing request data for POST requests. Since all
    /// data that can be supplied to POST request builders should be valid, this should never
    /// happen.
    #[fail(display = "Failed to serialize POST request data: {}", error)]
    SerializationError {
        /// The inner error contains the serialization error message from
        /// [`serde_json`](https://docs.rs/serde_json).
        error: serde_json::error::Error,
    },
    /// This error represents a successfully decoded bodhi server error message.
    #[fail(display = "Remote bodhi instance returned an error message: {}", error)]
    BodhiError {
        /// The inner [`BodhiError`](struct.BodhiError.html) contains the deserialized JSON error
        /// response from the server.
        error: BodhiError,
    },
    /// This error represents an unexpected response or query error from the bodhi instance.
    #[fail(display = "Failed to query bodhi service: {}", error)]
    ServiceError {
        /// The inner ServiceError contains more information about the type of failure that
        /// occurred, for example, malformed responses or network-related issues.
        error: ServiceError,
    },
}

impl From<serde_json::Error> for QueryError {
    fn from(error: serde_json::Error) -> Self {
        QueryError::DeserializationError { error }
    }
}

impl From<reqwest::Error> for QueryError {
    fn from(error: reqwest::Error) -> Self {
        QueryError::RequestError { error }
    }
}

impl From<ServiceError> for QueryError {
    fn from(error: ServiceError) -> Self {
        QueryError::ServiceError { error }
    }
}

/// This enum encapsulates the different ways in which requests to bodhi can fail.
#[derive(Debug, Fail)]
pub enum ServiceError {
    /// This error represents a network-related issue that occurred within
    /// [`reqwest`](https://docs.rs/reqwest).
    #[fail(display = "Failed to query bodhi instance: {}", error)]
    RequestError {
        /// The inner error contains the error passed from [`reqwest`](https://docs.rs/reqwest).
        error: reqwest::Error,
    },
    /// This error represents an issue with constructing the request URL from the base API URL
    /// and the query string.
    #[fail(display = "Failed to compute request URL: {}", error)]
    UrlParsingError {
        /// The inner error contains the error that occurred when parsing the URL.
        error: url::ParseError,
    },
    /// This error represents an issue where a response with an empty body was received (which is a
    /// server-side issue in bodhi, that sometimes happens under load).
    #[fail(display = "Received an empty response.")]
    EmptyResponseError,
    /// This error represents an unexpected error in the retry logic internal to
    /// [`retry`](https://docs.rs/retry).
    #[fail(display = "Retrying a failed request failed.")]
    RetryError,
}

impl From<reqwest::Error> for ServiceError {
    fn from(error: reqwest::Error) -> Self {
        ServiceError::RequestError { error }
    }
}

impl From<url::ParseError> for ServiceError {
    fn from(error: url::ParseError) -> Self {
        ServiceError::UrlParsingError { error }
    }
}
