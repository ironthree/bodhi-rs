use serde::Deserialize;

use crate::data::{BodhiError, Update};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

#[derive(Debug)]
pub struct UpdateIDQuery {
    id: String,
}

#[derive(Debug, Deserialize)]
struct UpdatePage {
    pub update: Update,
    pub can_edit: bool,
}

impl UpdateIDQuery {
    pub fn new(id: String) -> UpdateIDQuery {
        UpdateIDQuery { id }
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Update, String> {
        let path = format!("/updates/{}", self.id);

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let update: UpdatePage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(update.update)
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
