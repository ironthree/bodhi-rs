use serde::Deserialize;

use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

/// data type encapsulating (no) parameters for requesting a CSRF token
///
/// ```
/// use bodhi::CSRFQuery;
///
/// let query = CSRFQuery::new();
/// // let token = bodhi.request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/csrf.html>
#[derive(Debug, Default)]
pub struct CSRFQuery {}

#[derive(Debug, Deserialize)]
pub struct CSRFPage {
    csrf_token: String,
}

impl CSRFQuery {
    /// constructor for [`CSRFQuery`] (no mandatory or optional parameters)
    pub fn new() -> Self {
        Self::default()
    }
}

impl SingleRequest<CSRFPage, String> for CSRFQuery {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/csrf"))
    }

    fn parse(&self, string: &str) -> Result<CSRFPage, QueryError> {
        let page: CSRFPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: CSRFPage) -> String {
        page.csrf_token
    }
}
