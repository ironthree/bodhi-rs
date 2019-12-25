//! The contents of this module can be used to query a bodhi instance about the user accounts it
//! knows.
//!
//! The `UserNameQuery` returns exactly one User, if and only if a User with this username exists -
//! otherwise, it will return an error.
//!
//! The `UserQuery` can be used to execute more complex queries, for example filtering users by the
//! groups they are members of, or querying for users that are associated with a given set of
//! updates or packages.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::User;
use crate::error::{QueryError, ServiceError};
use crate::query::{Query, SinglePageQuery};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi for a specific user by their name. It will either return an
/// `Ok(User)` matching the specified name, return `Ok(None)` if it doesn't exist, or return an
/// `Err(String)` if another error occurred.
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::default().build().unwrap();
///
/// let comment = bodhi
///     .query(&bodhi::query::UserNameQuery::new(String::from("decathorpe")))
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/users.html#service-0>
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
    pub fn new(name: String) -> Self {
        UserNameQuery { name }
    }
}

impl SinglePageQuery<Option<User>> for UserNameQuery {
    fn path(&self) -> String {
        format!("/users/{}", self.name)
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        None
    }

    fn parse(string: String) -> Result<Option<User>, QueryError> {
        let user_page: UserPage = serde_json::from_str(&string)?;
        Ok(Some(user_page.user))
    }

    fn missing() -> Result<Option<User>, QueryError> {
        Ok(None)
    }
}

impl Query<Option<User>> for UserNameQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Option<User>, QueryError> {
        <Self as SinglePageQuery<Option<User>>>::query(self, bodhi)
    }
}

/// Use this for querying bodhi about a set of users with the given properties, which can be
/// specified with the builder pattern. Note that some options can be specified multiple times, and
/// users will be returned if any criteria match. This is consistent with both the web interface and
/// REST API behavior.
///
/// ```
/// # use bodhi::BodhiServiceBuilder;
/// # use bodhi::query::UserQuery;
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let users = bodhi
///     .query(&UserQuery::new().groups(String::from("provenpackager")))
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/users.html#service-1>
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
    pub fn new() -> Self {
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
    ///
    /// Can be specified multiple times.
    pub fn groups(mut self, group: String) -> Self {
        match &mut self.groups {
            Some(groups) => groups.push(group),
            None => self.groups = Some(vec![group]),
        }

        self
    }

    /// Restrict search to users *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: String) -> Self {
        self.like = Some(like);
        self
    }

    /// Restrict results to users with the given username.
    ///
    /// If this is the only required filter, consider using a `UserNameQuery` instead.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Restrict the returned results to users associated with the given package(s).
    ///
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> Self {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict search to users containing the given argument.
    pub fn search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }

    /// Restrict the returned results to users associated with the given update(s).
    ///
    /// Can be specified multiple times.
    pub fn updates(mut self, update: String) -> Self {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<User>, QueryError> {
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

impl Query<Vec<User>> for UserQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<User>, QueryError> {
        UserQuery::query(self, bodhi)
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
    fn new() -> Self {
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
}

impl SinglePageQuery<UserListPage> for UserPageQuery {
    fn path(&self) -> String {
        String::from("/users/")
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(groups) = &self.groups {
            args.insert("groups", groups.join(","));
        };

        if let Some(like) = &self.like {
            args.insert("like", like.to_owned());
        };

        if let Some(name) = &self.name {
            args.insert("name", name.to_owned());
        };

        if let Some(packages) = &self.packages {
            args.insert("packages", packages.join(","));
        };

        if let Some(search) = &self.search {
            args.insert("search", search.to_owned());
        };

        if let Some(updates) = &self.updates {
            args.insert("updates", updates.join(","));
        };

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        Some(args)
    }

    fn parse(string: String) -> Result<UserListPage, QueryError> {
        let user_page: UserListPage = serde_json::from_str(&string)?;
        Ok(user_page)
    }

    fn missing() -> Result<UserListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
