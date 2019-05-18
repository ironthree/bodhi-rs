use serde::Deserialize;

use crate::data::BodhiError;
use crate::service::BodhiService;

/// Use this for querying bodhi for a new CSRF token.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from(bodhi::FEDORA_BODHI_URL));
///
/// let token = bodhi::CSRFQuery::new().query(&bodhi).unwrap();
/// ```
#[derive(Debug, Default)]
pub struct CSRFQuery {}

#[derive(Debug, Deserialize)]
struct CSRFPage {
    csrf_token: String,
}

impl CSRFQuery {
    /// This method creates a new CSRF token query.
    pub fn new() -> CSRFQuery {
        CSRFQuery {}
    }

    /// This method will query the remote bodhi instance for a new CSRF token.
    /// It will return either an `Ok(String)` with the new token,
    /// or an `Err(String)` if an error occurred.
    pub fn query(self, bodhi: &BodhiService) -> Result<String, String> {
        let path = String::from("/csrf");

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let page: CSRFPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(page.csrf_token)
        } else {
            let error: BodhiError = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("Unexpected error message: {:?}", error));
                }
            };

            Err(format!("{:?}", error))
        }
    }
}
