//use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Override};
use crate::service::BodhiService;

//const DEFAULT_PAGE: i32 = 1;
//const DEFAULT_ROWS: i32 = 50;

#[derive(Debug)]
pub struct OverrideNVRQuery {
    nvr: String,
}

#[derive(Debug, Deserialize)]
struct OverridePage {
    pub r#override: Override,
}

impl OverrideNVRQuery {
    pub fn new(nvr: String) -> OverrideNVRQuery {
        OverrideNVRQuery { nvr }
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Override, String> {
        let path = format!("/overrides/{}", self.nvr);

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let override_page: OverridePage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(override_page.r#override)
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


#[derive(Debug, Deserialize)]
struct OverrideListPage {
    overrides: Vec<Override>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}
