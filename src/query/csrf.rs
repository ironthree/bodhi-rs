//! The contents of this module can be used to query a bodhi instance for a new CSRF token.

use serde::Deserialize;

use crate::error::{QueryError, ServiceError};
use crate::{BodhiService, Query, SinglePageQuery};

/// Use this for querying bodhi for a new CSRF token. It will return either an `Ok(String)` with the
/// new token, or an `Err(QueryError)` if an error occurred.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, CSRFQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let token = bodhi.query(&CSRFQuery::new()).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/csrf.html>
#[derive(Debug, Default)]
pub struct CSRFQuery {}

#[derive(Debug, Deserialize)]
struct CSRFPage {
    csrf_token: String,
}

impl CSRFQuery {
    /// This method creates a new [`CSRFQuery`](struct.CSRFQuery.html).
    pub fn new() -> Self {
        CSRFQuery {}
    }
}

impl SinglePageQuery<String> for CSRFQuery {
    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/csrf"))
    }

    fn parse(string: &str) -> Result<String, QueryError> {
        let page: CSRFPage = serde_json::from_str(string)?;
        Ok(page.csrf_token)
    }

    fn missing() -> Result<String, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}

impl Query<String> for CSRFQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<String, QueryError> {
        <Self as SinglePageQuery<String>>::query(self, bodhi)
    }
}
