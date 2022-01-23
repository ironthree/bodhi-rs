use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::client::DEFAULT_ROWS;
use crate::data::{BodhiDate, Comment};
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

/// data type encapsulating parameters for querying for a [`Comment`] by ID
///
/// If no comment with the specified ID is known to bodhi, a [`QueryError::NotFound`] error is
/// returned for the query.
///
/// ```
/// use bodhi::CommentIDQuery;
///
/// let query = CommentIDQuery::new(19999);
/// // let comment = bodhi.request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-0>
#[derive(Debug)]
pub struct CommentIDQuery {
    id: u32,
}

#[derive(Debug, Deserialize)]
pub struct CommentPage {
    comment: Comment,
}

impl CommentIDQuery {
    /// constructor for [`CommentIDQuery`] from a comment ID
    pub fn new(id: u32) -> Self {
        CommentIDQuery { id }
    }
}

impl SingleRequest<CommentPage, Comment> for CommentIDQuery {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/comments/{}", self.id))
    }

    fn parse(&self, string: &str) -> Result<CommentPage, QueryError> {
        let page: CommentPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: CommentPage) -> Comment {
        page.comment
    }
}


/// data type encapsulating parameters for querying [`Comment`]s
///
/// ```
/// use bodhi::CommentQuery;
///
/// let query = CommentQuery::new().update_owners(&["decathorpe"]);
/// // let comments = bodhi.paginated_request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-1>
#[derive(Default)]
pub struct CommentQuery<'a> {
    anonymous: Option<bool>,
    ignore_users: Option<&'a [&'a str]>,
    like: Option<&'a str>,
    packages: Option<&'a [&'a str]>,
    search: Option<&'a str>,
    since: Option<&'a BodhiDate>,
    update_owners: Option<&'a [&'a str]>,
    updates: Option<&'a [&'a str]>,
    users: Option<&'a [&'a str]>,

    // number of results per page
    rows_per_page: u32,
    // optional callback function for reporting progress
    callback: Option<Box<dyn Fn(u32, u32) + 'a>>,
}

impl<'a> Debug for CommentQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("CommentQuery")
            .field("anonymous", &self.anonymous)
            .field("ignore_users", &self.ignore_users)
            .field("like", &self.like)
            .field("packages", &self.packages)
            .field("search", &self.search)
            .field("since", &self.since)
            .field("update_owners", &self.update_owners)
            .field("updates", &self.updates)
            .field("users", &self.users)
            .field("rows_per_page", &self.rows_per_page)
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> CommentQuery<'a> {
    /// constructor for [`CommentQuery`] without any filters
    pub fn new() -> Self {
        CommentQuery {
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

    /// restrict query by excluding comments by certain users
    #[must_use]
    pub fn ignore_users(mut self, ignore_users: &'a [&'a str]) -> Self {
        self.ignore_users = Some(ignore_users);
        self
    }

    /// restrict query to comments where the text is "like" the given string (in the SQL sense)
    #[must_use]
    pub fn like(mut self, like: &'a str) -> CommentQuery {
        self.like = Some(like);
        self
    }

    /// restruct query to comments on updates for certain packages
    #[must_use]
    pub fn packages(mut self, packages: &'a [&'a str]) -> Self {
        self.packages = Some(packages);
        self
    }

    /// restrict query to comments matching a search keyword
    #[must_use]
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    /// restrict query to comments that have been posted since a specific date & time
    #[must_use]
    pub fn since(mut self, since: &'a BodhiDate) -> Self {
        self.since = Some(since);
        self
    }

    /// restrict query to comments on updates that have been submitted by certain users
    #[must_use]
    pub fn update_owners(mut self, update_owners: &'a [&'a str]) -> Self {
        self.update_owners = Some(update_owners);
        self
    }

    /// restrict query to comments on specific updates (identified by their update alias)
    #[must_use]
    pub fn updates(mut self, updates: &'a [&'a str]) -> Self {
        self.updates = Some(updates);
        self
    }

    /// restrict query to comments posted by specific users (identified by their username)
    #[must_use]
    pub fn users(mut self, users: &'a [&'a str]) -> Self {
        self.users = Some(users);
        self
    }
}


/// data type encapsulating parameters for querying specific [`CommentQuery`] result pages
#[derive(Debug, Serialize)]
pub struct CommentPageQuery<'a> {
    anonymous: Option<bool>,
    ignore_users: Option<&'a [&'a str]>,
    like: Option<&'a str>,
    packages: Option<&'a [&'a str]>,
    search: Option<&'a str>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    since: Option<&'a BodhiDate>,
    update_owners: Option<&'a [&'a str]>,
    updates: Option<&'a [&'a str]>,
    #[serde(rename = "user")]
    users: Option<&'a [&'a str]>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> CommentPageQuery<'a> {
    /// constructor for [`CommentPageQuery`] taking parameters from an existing [`CommentQuery`]
    pub fn from_query(query: &'a CommentQuery, page: u32) -> Self {
        CommentPageQuery {
            anonymous: query.anonymous,
            ignore_users: query.ignore_users,
            like: query.like,
            packages: query.packages,
            search: query.search,
            since: query.since,
            update_owners: query.update_owners,
            updates: query.updates,
            users: query.users,
            page,
            rows_per_page: query.rows_per_page,
        }
    }
}

impl<'a> SingleRequest<CommentListPage, Vec<Comment>> for CommentPageQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/comments/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(&self, string: &str) -> Result<CommentListPage, QueryError> {
        let comment_page: CommentListPage = serde_json::from_str(string)?;
        Ok(comment_page)
    }

    fn extract(&self, page: CommentListPage) -> Vec<Comment> {
        page.comments
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CommentListPage {
    comments: Vec<Comment>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

impl Pagination for CommentListPage {
    fn pages(&self) -> u32 {
        self.pages
    }
}

impl<'a> PaginatedRequest<CommentListPage, Vec<Comment>> for CommentQuery<'a> {
    fn page_request<'b>(&'b self, page: u32) -> Box<dyn SingleRequest<CommentListPage, Vec<Comment>> + 'b> {
        Box::new(CommentPageQuery::from_query(self, page))
    }

    fn callback(&self, page: u32, pages: u32) {
        if let Some(ref callback) = &self.callback {
            callback(page, pages)
        }
    }
}
