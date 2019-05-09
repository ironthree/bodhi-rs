//use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Release};
use crate::service::{BodhiService};
//use crate::service::{DEFAULT_PAGE, DEFAULT_ROWS};

#[derive(Debug)]
pub struct ReleaseNameQuery {
    name: String,
}

impl ReleaseNameQuery {
    pub fn new(name: String) -> ReleaseNameQuery {
        ReleaseNameQuery { name }
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Release, String> {
        let path = format!("/releases/{}", self.name);

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let release: Release = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(release)
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
struct ReleaseListPage {
    page: i32,
    pages: i32,
    releases: Vec<Release>,
    rows_per_page: i32,
    total: i32,
}
