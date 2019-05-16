use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, User};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi for a specific user by their name.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from("https://bodhi.fedoraproject.org"));
///
/// let comment = bodhi::UserNameQuery::new(String::from("decathorpe"))
///     .query(&bodhi).unwrap();
/// ```
#[derive(Debug)]
pub struct UserNameQuery {
    name: String,
}

#[derive(Debug, Deserialize)]
struct UserPage {
    user: User,
}

impl UserNameQuery {
    /// This method is the only way to create a new `UserNameQuery` instance.
    pub fn new(name: String) -> UserNameQuery {
        UserNameQuery { name }
    }

    /// This method will query the remote bodhi instance for the requested user by name,
    /// and will either return an `Ok(User)` matching the specified name,
    /// or return an `Err(String)` if they don't exist, or if another error occurred.
    ///
    /// TODO: return `Result<Option<User>, String>>` to distinguish "not found" from errors
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

/// Use this for querying bodhi about a set of users with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and users will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from("https://bodhi.fedoraproject.org"));
///
/// let users = bodhi::UserQuery::new()
///     .groups(String::from("provenpackager"))
///     .query(&bodhi).unwrap();
/// ```
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
    /// This method returns a new `UserQuery` with *no* filters set.
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

    /// Restrict the returned results to members of the given group(s).
    /// Can be specified multiple times.
    pub fn groups(mut self, group: String) -> UserQuery {
        match &mut self.groups {
            Some(groups) => groups.push(group),
            None => self.groups = Some(vec![group]),
        }

        self
    }

    /// Restrict search to users *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: String) -> UserQuery {
        self.like = Some(like);
        self
    }

    /// Restrict results to users with the given username.
    /// If this is the only required filter, consider using a `UserNameQuery` instead.
    pub fn name(mut self, name: String) -> UserQuery {
        self.name = Some(name);
        self
    }

    /// Restrict the returned results to users associated with the given package(s).
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> UserQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict search to users containing the given argument.
    pub fn search(mut self, search: String) -> UserQuery {
        self.search = Some(search);
        self
    }

    /// Restrict the returned results to users associated with the given update(s).
    /// Can be specified multiple times.
    pub fn updates(mut self, update: String) -> UserQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
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
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

#[derive(Debug)]
struct UserPageQuery {
    groups: Option<Vec<String>>,
    like: Option<String>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
    updates: Option<Vec<String>>,

    page: u32,
    rows_per_page: u32,
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
