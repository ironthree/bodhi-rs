// ! This module contains some common error types for wrapping networking-related issues,
// ! server-side issues, and client-side issues (including JSON deserialization problems).
use std::collections::HashMap;

use serde::Deserialize;

// This struct contains error messages that are deserialized from bodhi error responses.
#[derive(Debug, Deserialize, thiserror::Error)]
pub struct BodhiError {
    // This field contains a list of server-side error messages.
    pub errors: Vec<HashMap<String, String>>,
    // This field contains the server-side status message for the failure.
    pub status: String,
}

impl std::fmt::Display for BodhiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:#?}", self)
    }
}

// This enum encapsulates the different ways in which bodhi queries can fail.
#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    // This error represents an HTTP 404 response.
    #[error("Not found")]
    NotFound,
    // This error represents a network-related issue that occurred within
    // [`reqwest`](https://docs.rs/reqwest).
    #[error("Failed to query bodhi service: {error}")]
    RequestError {
        // The inner error contains the error passed from [`reqwest`](https://docs.rs/reqwest).
        error: reqwest::Error,
    },
    // This error represents an issue with deserializing JSON request data. If this ever happens,
    // it is almost certainly a bug in this crate.
    #[error("Failed to deserialize JSON response: {error}")]
    DeserializationError {
        // The inner error contains the deserialization error message from
        // [`serde_json`](https://docs.rs/serde_json).
        error: serde_json::Error,
    },
    // This error represents an issue with serializing request data for POST requests. Since all
    // data that can be supplied to POST request builders should be valid, this should never
    // happen.
    #[error("Failed to serialize POST request data: {error}")]
    SerializationError {
        // The inner error contains the serialization error message from
        // [`serde_json`](https://docs.rs/serde_json).
        error: serde_json::error::Error,
    },
    // This error represents a successfully decoded bodhi server error message.
    #[error("Remote bodhi instance returned an error message: {error}")]
    BodhiError {
        // The inner [`BodhiError`](struct.BodhiError.html) contains the deserialized JSON error
        // response from the server.
        error: BodhiError,
    },
    // This error represents an unexpected response or query error from the bodhi instance.
    #[error("Failed to query bodhi service: {error}")]
    ServiceError {
        // The inner ServiceError contains more information about the type of failure that
        // occurred, for example, malformed responses or network-related issues.
        error: ServiceError,
    },
    // This error represents an unexpected issue when constructing a query URL, probably due
    // to data that was not successfully deserialized into `x-www-urlencoded` format.
    #[error("Failed to construct `x-www-urlencoded` query string: {error}")]
    UrlEncodedError {
        // This inner error contains the deserialization error.
        error: String,
    },
    // This error represents some input data validation error.
    #[error("Invalid data: {error}")]
    InvalidDataError {
        // This inner error contains a the reason why the data was considered invalid.
        error: String,
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

impl From<serde_url_params::Error> for QueryError {
    fn from(error: serde_url_params::Error) -> Self {
        QueryError::UrlEncodedError {
            error: format!("{}", error),
        }
    }
}

// This enum encapsulates the different ways in which requests to bodhi can fail.
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    // This error represents a network-related issue that occurred within
    // [`reqwest`](https://docs.rs/reqwest).
    #[error("Failed to query bodhi instance: {error}")]
    RequestError {
        // The inner error contains the error passed from [`reqwest`](https://docs.rs/reqwest).
        #[from]
        error: reqwest::Error,
    },
    // This error represents an issue with constructing the request URL from the base API URL
    // and the query string.
    #[error("Failed to compute request URL: {error}")]
    UrlParsingError {
        // The inner error contains the error that occurred when parsing the URL.
        #[from]
        error: url::ParseError,
    },
    // This error represents an issue where a response with an empty body was received (which is a
    // server-side issue in bodhi, that sometimes happens under load).
    #[error("Received an empty response.")]
    EmptyResponseError,
}
