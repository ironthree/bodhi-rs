use std::collections::HashMap;

use failure::Fail;
use serde::Deserialize;

use crate::service::ServiceError;

/// This struct contains error messages that are deserialized from bodhi's error responses.
#[derive(Debug, Deserialize, Fail)]
pub struct BodhiError {
    pub errors: Vec<HashMap<String, String>>,
    pub status: String,
}

impl std::fmt::Display for BodhiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug, Fail)]
pub enum QueryError {
    #[fail(display = "Failed to query bodhi service: {}", error)]
    RequestError { error: reqwest::Error },
    #[fail(display = "Failed to deserialize JSON response: {}", error)]
    DeserializationError { error: serde_json::Error },
    #[fail(display = "Failed to serialize POST request data: {}", error)]
    SerializationError { error: serde_json::error::Error },
    #[fail(display = "Remote bodhi instance returned an error message: {}", error)]
    BodhiError { error: BodhiError },
    #[fail(display = "Failed to query bodhi service: {}", error)]
    ServiceError { error: ServiceError },
    #[fail(display = "POST request at remote bodhi instance failed: {}", error)]
    POSTError { error: String },
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
