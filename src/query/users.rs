//! The contents of this module can be used to query a bodhi instance about the user accounts it
//! knows.
//!
//! The [`UserNameQuery`](struct.UserNameQuery.html) returns exactly one
//! [`User`](../../data/types/struct.User.html), if and only if a `User` with this username exists -
//! otherwise, it will return an error.
//!
//! The [`UserQuery`](struct.UserQuery.html) can be used to execute more complex queries, for
//! example filtering users by the groups they are members of, or querying for users that are
//! associated with a given set of updates or packages.

use serde::{Deserialize, Serialize};

use crate::error::{QueryError, ServiceError};
use crate::service::DEFAULT_ROWS;
use crate::{BodhiService, Query, SinglePageQuery, User};

/// Use this for querying bodhi for a specific user by their name. It will either return an
/// `Ok(User)` matching the specified name, return `Ok(None)` if it doesn't exist, or return an
/// `Err(String)` if another error occurred.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, UserNameQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let comment = bodhi.query(&UserNameQuery::new(String::from("decathorpe"))).unwrap();
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
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/users/{}", self.name))
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
/// # use bodhi::{BodhiServiceBuilder, UserQuery};
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
    /// If this is the only required filter, consider using a
    /// [`UserNameQuery`](struct.UserNameQuery.html) instead.
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
            let query = self.page_query(page, DEFAULT_ROWS);
            let result = query.query(bodhi)?;

            users.extend(result.users);
            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(users)
    }

    fn page_query(&self, page: u32, rows_per_page: u32) -> UserPageQuery {
        UserPageQuery {
            groups: self.groups.as_ref(),
            like: self.like.as_ref(),
            name: self.name.as_ref(),
            packages: self.packages.as_ref(),
            search: self.search.as_ref(),
            updates: self.updates.as_ref(),
            page,
            rows_per_page,
        }
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

#[derive(Debug, Serialize)]
struct UserPageQuery<'a> {
    groups: Option<&'a Vec<String>>,
    like: Option<&'a String>,
    name: Option<&'a String>,
    packages: Option<&'a Vec<String>>,
    search: Option<&'a String>,
    updates: Option<&'a Vec<String>>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> SinglePageQuery<UserListPage> for UserPageQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/users/?{}", serde_url_params::to_string(self)?))
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
