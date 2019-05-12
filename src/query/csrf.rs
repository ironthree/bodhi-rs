use serde::Deserialize;

use crate::data::BodhiError;
use crate::service::BodhiService;

#[derive(Debug)]
pub struct CSRFQuery {}

#[derive(Debug, Deserialize)]
struct CSRFPage {
    csrf_token: String,
}

impl CSRFQuery {
    pub fn new() -> CSRFQuery {
        CSRFQuery {}
    }

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
