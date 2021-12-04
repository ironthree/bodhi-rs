// ! The contents of this module can be used to query a bodhi instance for a new CSRF token.

use serde::Deserialize;

use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/csrf.html>
#[derive(Debug, Default)]
pub struct CSRFQuery {}

#[derive(Debug, Deserialize)]
pub struct CSRFPage {
    csrf_token: String,
}

impl CSRFQuery {
    pub fn new() -> Self {
        CSRFQuery {}
    }
}

impl SingleRequest<CSRFPage, String> for CSRFQuery {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/csrf"))
    }

    fn body(&self) -> Option<String> {
        None
    }

    fn parse(&self, string: &str) -> Result<CSRFPage, QueryError> {
        let page: CSRFPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: CSRFPage) -> String {
        page.csrf_token
    }
}
