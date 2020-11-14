//! The contents of this module can be used to query a bodhi instance about the user accounts it
//! knows.
//!
//! The [`UserNameQuery`](struct.UserNameQuery.html) returns exactly one
//! [`User`](../../data/types/struct.User.html), if and only if a `User` with this username exists -
//! otherwise, it will return an error.
//!
//! The [`UserQuery`](struct.UserQuery.html) can be used to execute more complex queries, for
//! example filtering users by the groups they are members of, or querying for users that are
//! associated with a given set of updates.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::{QueryError, ServiceError};
use crate::service::DEFAULT_ROWS;
use crate::{BodhiService, Query, SinglePageQuery, User};

/// Use this for querying bodhi for a specific user by their name. It will either return an
/// `Ok(User)` matching the specified name, return `Ok(None)` if it doesn't exist, or return an
/// `Err(QueryError)` if another error occurred.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, UserNameQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// # #[cfg(feature = "online-tests")]
/// let comment = bodhi.query(UserNameQuery::new("decathorpe")).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/users.html#service-0>
#[derive(Debug)]
pub struct UserNameQuery<'a> {
    name: &'a str,
}

#[derive(Debug, Deserialize)]
struct UserPage {
    user: User,
}

impl<'a> UserNameQuery<'a> {
    /// This method is the only way to create a new `UserNameQuery` instance.
    pub fn new(name: &'a str) -> Self {
        UserNameQuery { name }
    }
}

impl<'a> SinglePageQuery<Option<User>> for UserNameQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/users/{}", self.name))
    }

    fn parse(string: &str) -> Result<Option<User>, QueryError> {
        let user_page: UserPage = serde_json::from_str(string)?;
        Ok(Some(user_page.user))
    }

    fn missing() -> Result<Option<User>, QueryError> {
        Ok(None)
    }
}

impl<'a> Query<Option<User>> for UserNameQuery<'a> {
    fn query(self, bodhi: &BodhiService) -> Result<Option<User>, QueryError> {
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
/// # #[cfg(feature = "online-tests")]
/// let users = bodhi.query(UserQuery::new().groups(&["provenpackager"])).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/users.html#service-1>
#[derive(Default)]
pub struct UserQuery<'a> {
    groups: Option<Vec<&'a str>>,
    like: Option<&'a str>,
    name: Option<&'a str>,
    search: Option<&'a str>,
    updates: Option<Vec<&'a str>>,

    /// optional callback function for reporting progress
    callback: Option<Box<dyn FnMut(u32, u32) + 'a>>,
}

impl<'a> Debug for UserQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "UserQuery {{ groups: {:?}, like: {:?}, name: {:?}, search: {:?}, updates: {:?} }}",
            &self.groups, &self.like, &self.name, &self.search, &self.updates,
        )
    }
}

impl<'a> UserQuery<'a> {
    /// This method returns a new `UserQuery` with *no* filters set.
    pub fn new() -> Self {
        UserQuery {
            groups: None,
            like: None,
            name: None,
            search: None,
            updates: None,
            callback: None,
        }
    }

    /// Add a callback function for reporting back query progress for long-running queries.
    /// The function will be called with the current page and the total number of pages for
    /// paginated queries.
    pub fn callback(mut self, fun: impl Fn(u32, u32) + 'a) -> Self {
        self.callback = Some(Box::new(fun));
        self
    }

    /// Restrict the returned results to members of the given group(s).
    pub fn groups(mut self, groups: &'a [&str]) -> Self {
        self.groups = Some(groups.to_vec());
        self
    }

    /// Restrict search to users *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    /// Restrict results to users with the given username.
    ///
    /// If this is the only required filter, consider using a
    /// [`UserNameQuery`](struct.UserNameQuery.html) instead.
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// Restrict search to users containing the given argument.
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    /// Restrict the returned results to users associated with the given update(s).
    pub fn updates(mut self, updates: &'a [&str]) -> Self {
        self.updates = Some(updates.to_vec());
        self
    }

    /// Query the remote bodhi instance with the given parameters.
    fn query(mut self, bodhi: &BodhiService) -> Result<Vec<User>, QueryError> {
        let mut users: Vec<User> = Vec::new();
        let mut page = 1;

        // initial progress: 0 out of some
        if let Some(ref mut fun) = self.callback {
            fun(0, 1);
        }

        loop {
            let query = self.page_query(page, DEFAULT_ROWS);
            let result = query.query(bodhi)?;

            if let Some(ref mut fun) = self.callback {
                fun(page, result.pages)
            }

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
            search: self.search.as_ref(),
            updates: self.updates.as_ref(),
            page,
            rows_per_page,
        }
    }
}

impl<'a> Query<Vec<User>> for UserQuery<'a> {
    fn query(self, bodhi: &BodhiService) -> Result<Vec<User>, QueryError> {
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
    groups: Option<&'a Vec<&'a str>>,
    like: Option<&'a &'a str>,
    name: Option<&'a &'a str>,
    search: Option<&'a &'a str>,
    updates: Option<&'a Vec<&'a str>>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> SinglePageQuery<UserListPage> for UserPageQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/users/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(string: &str) -> Result<UserListPage, QueryError> {
        let user_page: UserListPage = serde_json::from_str(string)?;
        Ok(user_page)
    }

    fn missing() -> Result<UserListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
