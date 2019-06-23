//! request a new CSRF token
//!
//! The contents of this module can be used to query a bodhi instance for a
//! new CSRF token.

use serde::Deserialize;

use crate::error::{BodhiError, QueryError};
use crate::service::BodhiService;

/// Use this for querying bodhi for a new CSRF token.
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::new(String::from(bodhi::FEDORA_BODHI_URL))
///     .build().unwrap();
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

    /// This method will query the remote bodhi instance for a new CSRF token.
    /// It will return either an `Ok(String)` with the new token,
    /// or an `Err(String)` if an error occurred.
    pub fn query(self, bodhi: &BodhiService) -> Result<String, QueryError> {
        let path = String::from("/csrf");

        let mut response = bodhi.get(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let result = response.text()?;
            let page: CSRFPage = serde_json::from_str(&result)?;

            Ok(page.csrf_token)
        } else {
            let result = response.text()?;
            let error: BodhiError = serde_json::from_str(&result)?;

            Err(QueryError::BodhiError { error })
        }
    }
}
