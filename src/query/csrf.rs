//! request a new CSRF token
//!
//! The contents of this module can be used to query a bodhi instance for a
//! new CSRF token.

use std::collections::HashMap;

use serde::Deserialize;

use crate::error::QueryError;
use crate::query::SinglePageQuery;
use crate::service::ServiceError;

/// Use this for querying bodhi for a new CSRF token.
/// It will return either an `Ok(String)` with the new token,
/// or an `Err(String)` if an error occurred.
/// ```
/// # use bodhi::query::SinglePageQuery;
///
/// let bodhi = bodhi::BodhiServiceBuilder::default().build().unwrap();
///
/// let token = bodhi::query::CSRFQuery::new().query(&bodhi).unwrap();
/// ```
#[derive(Debug, Default)]
pub struct CSRFQuery {}

#[derive(Debug, Deserialize)]
struct CSRFPage {
    csrf_token: String,
}

impl CSRFQuery {
    /// This method creates a new CSRF token query.
    pub fn new() -> Self {
        CSRFQuery {}
    }
}

impl SinglePageQuery for CSRFQuery {
    type Output = String;

    fn path(&self) -> String {
        String::from("/csrf")
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        None
    }

    fn missing() -> Result<Self::Output, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }

    fn parse(string: String) -> Result<String, QueryError> {
        let page: CSRFPage = serde_json::from_str(&string)?;
        Ok(page.csrf_token)
    }
}
