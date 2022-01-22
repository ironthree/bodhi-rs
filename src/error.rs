//! # custom error types and conversion methods
//!
//! This module contains the error types that are used for wrapping networking-related issues,
//! server-side issues, and client-side issues (including JSON deserialization problems).

use std::collections::HashMap;

use fedora::reqwest;
use fedora::url;
use serde::Deserialize;

/// error type representing an error message that was returned from a bodhi server
///
/// Some bodhi requests result in structured JSON error messages, and this struct is used for
/// deserializing those into Rust structs.
#[derive(Debug, Deserialize, thiserror::Error)]
pub struct BodhiError {
    /// list of structured server-side error messages (key-value-pairs)
    pub errors: Vec<HashMap<String, String>>,
    /// server-side status message
    pub status: String,
}

impl std::fmt::Display for BodhiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:#?}", self)
    }
}


/// error type representing an error that happened during the execution of a request
#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    /// request returned an HTTP 404 responses
    #[error("Not found")]
    NotFound,
    /// request returned an invalid / empty response
    #[error("Invalid / empty server response")]
    EmptyResponse,
    /// request failed due to networking issues
    #[error("Failed to query bodhi service: {error}")]
    RequestError {
        /// error returned by [`reqwest`]
        #[from]
        error: reqwest::Error,
    },
    /// failure to deserialize a JSON response
    ///
    /// If this error occurs, it is considered to be a bug in this crate.
    #[error("Failed to deserialize JSON response: {error}")]
    DeserializationError {
        /// error returned by [`serde_json`]
        error: serde_json::Error,
    },
    /// failure to serialize JSON request data
    ///
    /// If this error occurs, it is considered to be a bug in this crate.
    #[error("Failed to serialize POST request data: {error}")]
    SerializationError {
        /// error returned by [`serde_json`]
        error: serde_json::Error,
    },
    /// error parsing a string into a URL
    #[error("Failed to compute request URL: {error}")]
    UrlParsingError {
        /// error returned from [`url`]
        #[from]
        error: url::ParseError,
    },
    /// error representing an internal server failure
    #[error("Remote bodhi instance returned an error message: {error}")]
    BodhiError {
        /// error returned by the remove server
        error: BodhiError,
    },
    /// failure to serialize x-www-urlencoded request string
    #[error("Failed to construct `x-www-urlencoded` query string: {error}")]
    UrlEncodedError {
        /// error returned by [`serde_url_params`]
        #[from]
        error: serde_url_params::Error,
    },
    /// failure to validate input data
    #[error("Invalid data: {error}")]
    InvalidDataError {
        /// reason why data was considered invalid
        error: String,
    },
}

// The #[from] attribute for thiserror::Error can not be used for serde_json::Error, as there's two
// errors with this same inner error type.
impl From<serde_json::Error> for QueryError {
    fn from(error: serde_json::Error) -> Self {
        QueryError::DeserializationError { error }
    }
}
