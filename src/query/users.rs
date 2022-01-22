// ! The contents of this module can be used to query a bodhi instance about the user accounts it
// ! knows.
// !
// ! The [`UserNameQuery`](struct.UserNameQuery.html) returns exactly one
// ! [`User`](../../data/types/struct.User.html), if and only if a `User` with this username exists
// - ! otherwise, it will return an error.
// !
// ! The [`UserQuery`](struct.UserQuery.html) can be used to execute more complex queries, for
// ! example filtering users by the groups they are members of, or querying for users that are
// ! associated with a given set of updates.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::client::DEFAULT_ROWS;
use crate::data::User;
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

// Use this for querying bodhi for a specific user by their name. It will either return an
// `Ok(User)` matching the specified name, return `Ok(None)` if it doesn't exist, or return an
// `Err(QueryError)` if another error occurred.
//
// ```
// # use bodhi::{BodhiServiceBuilder, UserNameQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let comment = bodhi.query(UserNameQuery::new("decathorpe")).unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/users.html#service-0>
#[derive(Debug)]
pub struct UserNameQuery<'a> {
    name: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct UserPage {
    user: User,
}

impl<'a> UserNameQuery<'a> {
    // This method is the only way to create a new `UserNameQuery` instance.
    pub fn new(name: &'a str) -> Self {
        UserNameQuery { name }
    }
}

impl<'a> SingleRequest<UserPage, User> for UserNameQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/users/{}", self.name))
    }

    fn parse(&self, string: &str) -> Result<UserPage, QueryError> {
        let page: UserPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: UserPage) -> User {
        page.user
    }
}

// Use this for querying bodhi about a set of users with the given properties, which can be
// specified with the builder pattern. Note that some options can be specified multiple times, and
// users will be returned if any criteria match. This is consistent with both the web interface and
// REST API behavior.
//
// ```
// # use bodhi::{BodhiServiceBuilder, UserQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let users = bodhi.query(UserQuery::new().groups(vec!["provenpackager"])).unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/users.html#service-1>
#[derive(Default)]
pub struct UserQuery<'a> {
    groups: Option<Vec<&'a str>>,
    like: Option<&'a str>,
    name: Option<&'a str>,
    search: Option<&'a str>,
    updates: Option<Vec<&'a str>>,

    // number of results per page
    rows_per_page: u32,
    // optional callback function for reporting progress
    callback: Option<Box<dyn Fn(u32, u32) + 'a>>,
}

impl<'a> Debug for UserQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("UserQuery")
            .field("groups", &self.groups)
            .field("like", &self.like)
            .field("name", &self.name)
            .field("search", &self.search)
            .field("updates", &self.updates)
            .field("rows_per_page", &self.rows_per_page)
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> UserQuery<'a> {
    // This method returns a new `UserQuery` with *no* filters set.
    pub fn new() -> Self {
        UserQuery {
            rows_per_page: DEFAULT_ROWS,
            ..Default::default()
        }
    }

    // Override the maximum number of results per page (capped at 100 server-side).
    #[must_use]
    pub fn rows_per_page(mut self, rows_per_page: u32) -> Self {
        self.rows_per_page = rows_per_page;
        self
    }

    // Add a callback function for reporting back query progress for long-running queries.
    // The function will be called with the current page and the total number of pages for
    // paginated queries.
    #[must_use]
    pub fn callback(mut self, fun: impl Fn(u32, u32) + 'a) -> Self {
        self.callback = Some(Box::new(fun));
        self
    }

    // Restrict the returned results to members of the given group(s).
    #[must_use]
    pub fn groups(mut self, groups: Vec<&'a str>) -> Self {
        self.groups = Some(groups);
        self
    }

    // Restrict search to users *like* the given argument (in the SQL sense).
    #[must_use]
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    // Restrict results to users with the given username.
    //
    // If this is the only required filter, consider using a
    // [`UserNameQuery`](struct.UserNameQuery.html) instead.
    #[must_use]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    // Restrict search to users containing the given argument.
    #[must_use]
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    // Restrict the returned results to users associated with the given update(s).
    #[must_use]
    pub fn updates(mut self, updates: Vec<&'a str>) -> Self {
        self.updates = Some(updates);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct UserPageQuery<'a> {
    groups: Option<&'a Vec<&'a str>>,
    like: Option<&'a str>,
    name: Option<&'a str>,
    search: Option<&'a str>,
    updates: Option<&'a Vec<&'a str>>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> UserPageQuery<'a> {
    pub fn from_query(query: &'a UserQuery, page: u32) -> Self {
        UserPageQuery {
            groups: query.groups.as_ref(),
            like: query.like,
            name: query.name,
            search: query.search,
            updates: query.updates.as_ref(),
            page,
            rows_per_page: DEFAULT_ROWS,
        }
    }
}

impl<'a> SingleRequest<UserListPage, Vec<User>> for UserPageQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/users/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(&self, string: &str) -> Result<UserListPage, QueryError> {
        let page: UserListPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: UserListPage) -> Vec<User> {
        page.users
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UserListPage {
    users: Vec<User>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

impl Pagination for UserListPage {
    fn pages(&self) -> u32 {
        self.pages
    }
}

impl<'a> PaginatedRequest<UserListPage, Vec<User>> for UserQuery<'a> {
    fn page_request<'b>(&'b self, page: u32) -> Box<dyn SingleRequest<UserListPage, Vec<User>> + 'b> {
        Box::new(UserPageQuery::from_query(self, page))
    }

    fn callback(&self, page: u32, pages: u32) {
        if let Some(ref callback) = &self.callback {
            callback(page, pages)
        }
    }
}
