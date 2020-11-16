//! The contents of this module can be used to query a bodhi instance about existing comments.
//!
//! The [`CommentIDQuery`](struct.CommentIDQuery.html) returns exactly one
//! [`Comment`](../../data/struct.Comment.html), if and only if a Comment with the given integer ID
//! exists - otherwise, it will return an error.
//!
//! The [`CommentQuery`](struct.CommentQuery.html) can be used to execute more complex queries, for
//! example filtering comments that are associated with a set of updates or packages, or query
//! comments made by certain users, or filed against updates that were created by specific users.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::{QueryError, ServiceError};
use crate::service::DEFAULT_ROWS;
use crate::{BodhiDate, BodhiService, Comment, Query, SinglePageQuery};

/// Use this for querying bodhi for a specific comment by its ID. It will either return an
/// `Ok(Some(Comment))` matching the specified ID, return `Ok(None)` if it doesn't exist, or return
/// an `Err(QueryError)` if another error occurred.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, CommentIDQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// # #[cfg(feature = "online-tests")]
/// let comment = bodhi.query(CommentIDQuery::new(19999)).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-0>
#[derive(Debug)]
pub struct CommentIDQuery {
    id: u32,
}

#[derive(Debug, Deserialize)]
struct CommentPage {
    comment: Comment,
}

impl CommentIDQuery {
    /// This method is the only way to create a new [`CommentIDQuery`](struct.CommentIDQuery.html)
    /// instance.
    pub fn new(id: u32) -> Self {
        CommentIDQuery { id }
    }
}

impl SinglePageQuery<Option<Comment>> for CommentIDQuery {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/comments/{}", self.id))
    }

    fn parse(string: &str) -> Result<Option<Comment>, QueryError> {
        let comment: CommentPage = serde_json::from_str(string)?;
        Ok(Some(comment.comment))
    }

    fn missing() -> Result<Option<Comment>, QueryError> {
        Ok(None)
    }
}

impl Query<Option<Comment>> for CommentIDQuery {
    fn query(self, bodhi: &BodhiService) -> Result<Option<Comment>, QueryError> {
        <Self as SinglePageQuery<Option<Comment>>>::query(self, bodhi)
    }
}

/// Use this for querying bodhi about a set of comments with the given properties, which can be
/// specified with the builder pattern. Note that some options can be specified multiple times, and
/// comments will be returned if any criteria match. This is consistent with both the web interface
/// and REST API behavior.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, CommentQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// # #[cfg(feature = "online-tests")]
/// let comments = bodhi
///     .query(CommentQuery::new().users(vec!["decathorpe"]).packages(vec!["rust"]))
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-1>
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

    /// optional callback function for reporting progress
    callback: Option<Box<dyn FnMut(u32, u32) + 'a>>,
}

impl<'a> Debug for CommentQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "CommentQuery {{ anonymous: {:?}, ignore_users: {:?}, like: {:?}, packages: {:?}, search: {:?}, since: {:?}, update_owners: {:?}, updates: {:?}, users: {:?} }}",
            self.anonymous,
            self.ignore_users,
            self.like,
            self.packages,
            self.search,
            self.since,
            self.update_owners,
            self.updates,
            self.users,
        )
    }
}

impl<'a> CommentQuery<'a> {
    /// This method returns a new [`CommentQuery`](struct.CommentQuery.html) with *no* filters set.
    pub fn new() -> Self {
        CommentQuery {
            anonymous: None,
            ignore_users: None,
            like: None,
            packages: None,
            search: None,
            since: None,
            update_owners: None,
            updates: None,
            users: None,
            callback: None,
        }
    }

    /// Add a callback function for reporting back query progress for long-running queries.
    /// The function will be called with the current page and the total number of pages for
    /// paginated queries.
    pub fn callback(mut self, fun: impl FnMut(u32, u32) + 'a) -> Self {
        self.callback = Some(Box::new(fun));
        self
    }

    /// Restrict the returned results to (not) anonymous comments.
    ///
    /// **NOTE**: Anonymous comments are no longer supported as of bodhi 4.0.
    pub fn anonymous(mut self, anonymous: bool) -> Self {
        self.anonymous = Some(anonymous);
        self
    }

    /// Restrict results to ignore comments by certain users.
    pub fn ignore_users(mut self, ignore_users: Vec<&'a str>) -> Self {
        self.ignore_users = Some(ignore_users);
        self
    }

    /// Restrict search to comments *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: &'a str) -> CommentQuery {
        self.like = Some(like);
        self
    }

    /// Restrict the returned results to comments filed against updates for the given package(s).
    pub fn packages(mut self, packages: Vec<&'a str>) -> Self {
        self.packages = Some(packages);
        self
    }

    /// Restrict search to comments containing the given argument.
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    /// Restrict the returned results to comments filed since the given date and time.
    pub fn since(mut self, since: &'a BodhiDate) -> Self {
        self.since = Some(since);
        self
    }

    /// Restrict the returned results to comments filed against updates created by the specified
    /// user(s).
    pub fn update_owners(mut self, update_owners: Vec<&'a str>) -> Self {
        self.update_owners = Some(update_owners);
        self
    }

    /// Restrict the returned results to comments filed against the given update(s).
    pub fn updates(mut self, updates: Vec<&'a str>) -> Self {
        self.updates = Some(updates);
        self
    }

    /// Restrict the returned results to comments filed by the given user(s).
    pub fn users(mut self, users: Vec<&'a str>) -> Self {
        self.users = Some(users);
        self
    }

    /// Query the remote bodhi instance with the given parameters.
    fn query(mut self, bodhi: &BodhiService) -> Result<Vec<Comment>, QueryError> {
        let mut comments: Vec<Comment> = Vec::new();
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

            comments.extend(result.comments);
            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(comments)
    }

    fn page_query(&self, page: u32, rows_per_page: u32) -> CommentPageQuery {
        CommentPageQuery {
            anonymous: self.anonymous,
            ignore_users: self.ignore_users.as_ref(),
            like: self.like,
            packages: self.packages.as_ref(),
            search: self.search,
            since: self.since,
            update_owners: self.update_owners.as_ref(),
            updates: self.updates.as_ref(),
            users: self.users.as_ref(),
            page,
            rows_per_page,
        }
    }
}

impl<'a> Query<Vec<Comment>> for CommentQuery<'a> {
    fn query(self, bodhi: &BodhiService) -> Result<Vec<Comment>, QueryError> {
        CommentQuery::query(self, bodhi)
    }
}

#[derive(Debug, Deserialize)]
struct CommentListPage {
    comments: Vec<Comment>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

#[derive(Debug, Serialize)]
struct CommentPageQuery<'a> {
    anonymous: Option<bool>,
    ignore_users: Option<&'a Vec<&'a str>>,
    like: Option<&'a str>,
    packages: Option<&'a Vec<&'a str>>,
    search: Option<&'a str>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    since: Option<&'a BodhiDate>,
    update_owners: Option<&'a Vec<&'a str>>,
    updates: Option<&'a Vec<&'a str>>,
    #[serde(rename = "user")]
    users: Option<&'a Vec<&'a str>>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> SinglePageQuery<CommentListPage> for CommentPageQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/comments/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(string: &str) -> Result<CommentListPage, QueryError> {
        let comment_page: CommentListPage = serde_json::from_str(string)?;
        Ok(comment_page)
    }

    fn missing() -> Result<CommentListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
