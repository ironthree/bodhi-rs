// ! The contents of this module can be used to query a bodhi instance about existing comments.
// !
// ! The [`CommentIDQuery`](struct.CommentIDQuery.html) returns exactly one
// ! [`Comment`](../../data/struct.Comment.html), if and only if a Comment with the given integer ID
// ! exists - otherwise, it will return an error.
// !
// ! The [`CommentQuery`](struct.CommentQuery.html) can be used to execute more complex queries, for
// ! example filtering comments that are associated with a set of updates or packages, or query
// ! comments made by certain users, or filed against updates that were created by specific users.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::data::{BodhiDate, Comment};
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};
use crate::service::DEFAULT_ROWS;

// Use this for querying bodhi for a specific comment by its ID. It will either return an
// `Ok(Some(Comment))` matching the specified ID, return `Ok(None)` if it doesn't exist, or return
// an `Err(QueryError)` if another error occurred.
//
// ```
// # use bodhi::{BodhiServiceBuilder, CommentIDQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let comment = bodhi.query(CommentIDQuery::new(19999)).unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-0>
#[derive(Debug)]
pub struct CommentIDQuery {
    id: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CommentPage {
    comment: Comment,
}

impl CommentIDQuery {
    // This method is the only way to create a new [`CommentIDQuery`](struct.CommentIDQuery.html)
    // instance.
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

// Use this for querying bodhi about a set of comments with the given properties, which can be
// specified with the builder pattern. Note that some options can be specified multiple times, and
// comments will be returned if any criteria match. This is consistent with both the web interface
// and REST API behavior.
//
// ```
// # use bodhi::{BodhiServiceBuilder, CommentQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let comments = bodhi
//     .query(CommentQuery::new().users(vec!["decathorpe"]).packages(vec!["rust"]))
//     .unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-1>
#[derive(Default)]
pub struct CommentQuery<'a> {
    anonymous: Option<bool>,
    ignore_users: Option<Vec<&'a str>>,
    like: Option<&'a str>,
    packages: Option<Vec<&'a str>>,
    search: Option<&'a str>,
    since: Option<&'a BodhiDate>,
    update_owners: Option<Vec<&'a str>>,
    updates: Option<Vec<&'a str>>,
    users: Option<Vec<&'a str>>,

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
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> CommentQuery<'a> {
    // This method returns a new [`CommentQuery`](struct.CommentQuery.html) with *no* filters set.
    pub fn new() -> Self {
        Self::default()
    }

    // Add a callback function for reporting back query progress for long-running queries.
    // The function will be called with the current page and the total number of pages for
    // paginated queries.
    pub fn callback(mut self, fun: impl Fn(u32, u32) + 'a) -> Self {
        self.callback = Some(Box::new(fun));
        self
    }

    // Restrict the returned results to (not) anonymous comments.
    //
    // **NOTE**: Anonymous comments are no longer supported as of bodhi 4.0.
    // FIXME: remove this
    pub fn anonymous(mut self, anonymous: bool) -> Self {
        self.anonymous = Some(anonymous);
        self
    }

    // Restrict results to ignore comments by certain users.
    pub fn ignore_users(mut self, ignore_users: Vec<&'a str>) -> Self {
        self.ignore_users = Some(ignore_users);
        self
    }

    // Restrict search to comments *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: &'a str) -> CommentQuery {
        self.like = Some(like);
        self
    }

    // Restrict the returned results to comments filed against updates for the given package(s).
    pub fn packages(mut self, packages: Vec<&'a str>) -> Self {
        self.packages = Some(packages);
        self
    }

    // Restrict search to comments containing the given argument.
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    // Restrict the returned results to comments filed since the given date and time.
    pub fn since(mut self, since: &'a BodhiDate) -> Self {
        self.since = Some(since);
        self
    }

    // Restrict the returned results to comments filed against updates created by the specified
    // user(s).
    pub fn update_owners(mut self, update_owners: Vec<&'a str>) -> Self {
        self.update_owners = Some(update_owners);
        self
    }

    // Restrict the returned results to comments filed against the given update(s).
    pub fn updates(mut self, updates: Vec<&'a str>) -> Self {
        self.updates = Some(updates);
        self
    }

    // Restrict the returned results to comments filed by the given user(s).
    pub fn users(mut self, users: Vec<&'a str>) -> Self {
        self.users = Some(users);
        self
    }
}

#[derive(Debug, Serialize)]
struct CommentPageQuery {
    anonymous: Option<bool>,
    ignore_users: Option<Vec<String>>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
    #[serde(with = "crate::option_bodhi_date_format")]
    since: Option<BodhiDate>,
    update_owners: Option<Vec<String>>,
    updates: Option<Vec<String>>,
    #[serde(rename = "user")]
    users: Option<Vec<String>>,

    page: u32,
    rows_per_page: u32,
}

impl SingleRequest<CommentListPage, Vec<Comment>> for CommentPageQuery {
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
    fn page_request(&self, page: u32) -> Box<dyn SingleRequest<CommentListPage, Vec<Comment>>> {
        Box::new(CommentPageQuery {
            anonymous: self.anonymous,
            ignore_users: self
                .ignore_users
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            like: self.like.map(|s| s.to_owned()),
            packages: self
                .packages
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            search: self.search.map(|s| s.to_owned()),
            since: self.since.map(|s| s.clone()),
            update_owners: self
                .update_owners
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            updates: self
                .updates
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            users: self.users.as_ref().map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            page,
            rows_per_page: DEFAULT_ROWS,
        })
    }

    fn callback(&self, page: u32, pages: u32) {
        if let Some(ref callback) = &self.callback {
            callback(page, pages)
        }
    }
}
