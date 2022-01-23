use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::client::DEFAULT_ROWS;
use crate::data::User;
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

/// data type encapsulating parameters for querying for a [`User`] by name
///
/// If no user with the specified name is known to bodhi, a [`QueryError::NotFound`] error is
/// returned for the query.
///
/// ```
/// use bodhi::UserNameQuery;
///
/// let query = UserNameQuery::new("decathorpe");
/// // let user = bodhi.request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/users.html#service-0>
#[derive(Debug)]
pub struct UserNameQuery<'a> {
    name: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct UserPage {
    user: User,
}

impl<'a> UserNameQuery<'a> {
    /// constructor for [`UserNameQuery`] from a username
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


/// data type encapsulating parameters for querying [`User`]s
///
/// ```
/// use bodhi::UserQuery;
///
/// let query = UserQuery::new().groups(&["provenpackager"]);
/// // let users = bodhi.paginated_request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/users.html#service-1>
#[derive(Default)]
pub struct UserQuery<'a> {
    groups: Option<&'a [&'a str]>,
    like: Option<&'a str>,
    name: Option<&'a str>,
    search: Option<&'a str>,
    updates: Option<&'a [&'a str]>,

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
    /// constructor for [`UserQuery`] without any filters
    pub fn new() -> Self {
        UserQuery {
            rows_per_page: DEFAULT_ROWS,
            ..Default::default()
        }
    }

    /// override the default number of results per page
    #[must_use]
    pub fn rows_per_page(mut self, rows_per_page: u32) -> Self {
        self.rows_per_page = rows_per_page;
        self
    }

    /// add callback function for progress reporting during long-running queries
    ///
    /// The specified function will be called with the current result page and the number of total
    /// pages as arguments.
    #[must_use]
    pub fn callback(mut self, fun: impl Fn(u32, u32) + 'a) -> Self {
        self.callback = Some(Box::new(fun));
        self
    }

    /// restrict query to users that are members of the specified groups
    #[must_use]
    pub fn groups(mut self, groups: &'a [&'a str]) -> Self {
        self.groups = Some(groups);
        self
    }

    /// restrict query to users with usernames "like" the given string (in the SQL sense)
    #[must_use]
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    /// restrict query to users matching a specific username
    ///
    /// If this is the only parameter, consider using a [`UserNameQuery`] instead.
    #[must_use]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// restrict query to users with usernames that match a search keyword
    #[must_use]
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    /// restrict query to users to submitted of specific updates (identified by their update alias)
    #[must_use]
    pub fn updates(mut self, updates: &'a [&'a str]) -> Self {
        self.updates = Some(updates);
        self
    }
}


/// data type encapsulating parameters for querying specific [`UserQuery`] result pages
#[derive(Debug, Serialize)]
pub struct UserPageQuery<'a> {
    groups: Option<&'a [&'a str]>,
    like: Option<&'a str>,
    name: Option<&'a str>,
    search: Option<&'a str>,
    updates: Option<&'a [&'a str]>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> UserPageQuery<'a> {
    /// constructor for [`UserPageQuery`] taking parameters from an existing [`UserQuery`]
    pub fn from_query(query: &'a UserQuery, page: u32) -> Self {
        UserPageQuery {
            groups: query.groups,
            like: query.like,
            name: query.name,
            search: query.search,
            updates: query.updates,
            page,
            rows_per_page: query.rows_per_page,
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
