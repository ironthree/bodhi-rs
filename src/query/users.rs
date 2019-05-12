// use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, User};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

#[derive(Debug)]
pub struct UserNameQuery {
    name: String,
}

#[derive(Debug, Deserialize)]
struct UserPage {
    user: User,
}

impl UserNameQuery {
    pub fn new(name: String) -> UserNameQuery {
        UserNameQuery { name }
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<User, String> {
        let path = format!("/users/{}", self.name);

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let user: UserPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(user.user)
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
struct UserListPage {
    users: Vec<User>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}
