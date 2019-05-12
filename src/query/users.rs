use std::collections::HashMap;

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

#[derive(Debug, Default)]
pub struct UserQuery {
    groups: Option<Vec<String>>,
    like: Option<String>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
    updates: Option<Vec<String>>,
}

impl UserQuery {
    pub fn new() -> UserQuery {
        UserQuery {
            groups: None,
            like: None,
            name: None,
            packages: None,
            search: None,
            updates: None,
        }
    }

    pub fn groups(mut self, group: String) -> UserQuery {
        match &mut self.groups {
            Some(groups) => groups.push(group),
            None => self.groups = Some(vec![group]),
        }

        self
    }

    pub fn like(mut self, like: String) -> UserQuery {
        self.like = Some(like);
        self
    }

    pub fn name(mut self, name: String) -> UserQuery {
        self.name = Some(name);
        self
    }

    pub fn packages(mut self, package: String) -> UserQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    pub fn search(mut self, search: String) -> UserQuery {
        self.search = Some(search);
        self
    }

    pub fn updates(mut self, update: String) -> UserQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<User>, String> {
        let mut users: Vec<User> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = UserPageQuery::new();
            query.page = page;

            query.groups = self.groups.clone();
            query.like = self.like.clone();
            query.name = self.name.clone();
            query.packages = self.packages.clone();
            query.search = self.search.clone();
            query.updates = self.updates.clone();

            let result = query.query(bodhi)?;
            users.extend(result.users);

            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(users)
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

#[derive(Debug)]
struct UserPageQuery {
    groups: Option<Vec<String>>,
    like: Option<String>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
    updates: Option<Vec<String>>,

    page: i32,
    rows_per_page: i32,
}

impl UserPageQuery {
    fn new() -> UserPageQuery {
        UserPageQuery {
            groups: None,
            like: None,
            name: None,
            packages: None,
            search: None,
            updates: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
        }
    }

    fn query(self, bodhi: &BodhiService) -> Result<UserListPage, String> {
        let path = String::from("/users/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(groups) = self.groups {
            args.insert("groups", groups.join(","));
        };

        if let Some(like) = self.like {
            args.insert("like", like);
        };

        if let Some(name) = self.name {
            args.insert("name", name);
        };

        if let Some(packages) = self.packages {
            args.insert("packages", packages.join(","));
        };

        if let Some(search) = self.search {
            args.insert("search", search);
        };

        if let Some(updates) = self.updates {
            args.insert("updates", updates.join(","));
        };

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        let mut response = bodhi.request(&path, Some(args))?;
        let status = response.status();

        if status.is_success() {
            let users: UserListPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(users)
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
