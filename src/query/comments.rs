//! The contents of this module can be used to query a bodhi instance about existing comments.
//!
//! The [`CommentIDQuery`](struct.CommentIDQuery.html) returns exactly one
//! [`Comment`](../../data/struct.Comment.html), if and only if a Comment with the given integer ID
//! exists - otherwise, it will return an error.
//!
//! The [`CommentQuery`](struct.CommentQuery.html) can be used to execute more complex queries, for
//! example filtering comments that are associated with a set of updates or packages, or query
//! comments made by certain users, or filed against updates that were created by specific users.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::Comment;
use crate::error::QueryError;
use crate::query::{Query, SinglePageQuery};
use crate::service::{BodhiService, ServiceError, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi for a specific comment by its ID. It will either return an
/// `Ok(Some(Comment))` matching the specified ID, return `Ok(None)` if it doesn't exist, or return
/// an `Err(String)` if another error occurred.
///
/// ```
/// # use bodhi::BodhiServiceBuilder;
/// # use bodhi::query::CommentIDQuery;
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let comment = bodhi.query(&CommentIDQuery::new(19999)).unwrap();
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
    fn path(&self) -> String {
        format!("/comments/{}", self.id)
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        None
    }

    fn parse(string: String) -> Result<Option<Comment>, QueryError> {
        let comment: CommentPage = serde_json::from_str(&string)?;
        Ok(Some(comment.comment))
    }

    fn missing() -> Result<Option<Comment>, QueryError> {
        Ok(None)
    }
}

impl Query<Option<Comment>> for CommentIDQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Option<Comment>, QueryError> {
        <Self as SinglePageQuery<Option<Comment>>>::query(self, bodhi)
    }
}

/// Use this for querying bodhi about a set of comments with the given properties, which can be
/// specified with the builder pattern. Note that some options can be specified multiple times, and
/// comments will be returned if any criteria match. This is consistent with both the web interface
/// and REST API behavior.
///
/// ```
/// # use bodhi::BodhiServiceBuilder;
/// # use bodhi::query::CommentQuery;
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let comments = bodhi
///     .query(
///         &CommentQuery::new()
///             .users(String::from("decathorpe"))
///             .packages(String::from("rust")),
///     )
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-1>
#[derive(Debug, Default)]
pub struct CommentQuery {
    anonymous: Option<bool>,
    ignore_users: Option<Vec<String>>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
    since: Option<String>,
    update_owners: Option<Vec<String>>,
    updates: Option<Vec<String>>,
    users: Option<Vec<String>>,
}

impl CommentQuery {
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
        }
    }

    /// Restrict the returned results to (not) anonymous comments.
    ///
    /// **NOTE**: Anonymous comments are no longer supported as of bodhi 4.0.
    pub fn anonymous(mut self, anonymous: bool) -> Self {
        self.anonymous = Some(anonymous);
        self
    }

    /// Restrict results to ignore comments by certain users.
    ///
    /// Can be specified multiple times.
    pub fn ignore_users(mut self, ignore_user: String) -> Self {
        match &mut self.ignore_users {
            Some(ignore_users) => ignore_users.push(ignore_user),
            None => self.ignore_users = Some(vec![ignore_user]),
        }

        self
    }

    /// Restrict search to comments *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: String) -> CommentQuery {
        self.like = Some(like);
        self
    }

    /// Restrict the returned results to comments filed against updates for the given package(s).
    ///
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> Self {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict search to comments containing the given argument.
    pub fn search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }

    /// Restrict the returned results to comments filed since the given date and time.
    pub fn since(mut self, since: String) -> Self {
        self.since = Some(since);
        self
    }

    /// Restrict the returned results to comments filed against updates created by the specified
    /// user(s).
    ///
    /// Can be specified multiple times.
    pub fn update_owners(mut self, update_owner: String) -> Self {
        match &mut self.update_owners {
            Some(update_owners) => update_owners.push(update_owner),
            None => self.update_owners = Some(vec![update_owner]),
        }

        self
    }

    /// Restrict the returned results to comments filed against the given update(s).
    ///
    /// Can be specified multiple times.
    pub fn updates(mut self, update: String) -> Self {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    /// Restrict the returned results to comments filed by the given user(s).
    ///
    /// Can be specified multiple times.
    pub fn users(mut self, user: String) -> Self {
        match &mut self.users {
            Some(users) => users.push(user),
            None => self.users = Some(vec![user]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Comment>, QueryError> {
        let mut comments: Vec<Comment> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = CommentPageQuery::new();
            query.page = page;

            query.anonymous = self.anonymous;
            query.ignore_users = self.ignore_users.clone();
            query.like = self.like.clone();
            query.packages = self.packages.clone();
            query.search = self.search.clone();
            query.update_owners = self.update_owners.clone();
            query.updates = self.updates.clone();
            query.users = self.users.clone();

            let result = query.query(bodhi)?;
            comments.extend(result.comments);

            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(comments)
    }
}

impl Query<Vec<Comment>> for CommentQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Comment>, QueryError> {
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

#[derive(Debug)]
struct CommentPageQuery {
    anonymous: Option<bool>,
    ignore_users: Option<Vec<String>>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
    since: Option<String>,
    update_owners: Option<Vec<String>>,
    updates: Option<Vec<String>>,
    users: Option<Vec<String>>,

    page: u32,
    rows_per_page: u32,
}

impl CommentPageQuery {
    fn new() -> Self {
        CommentPageQuery {
            anonymous: None,
            ignore_users: None,
            like: None,
            packages: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
            search: None,
            since: None,
            update_owners: None,
            updates: None,
            users: None,
        }
    }
}

impl SinglePageQuery<CommentListPage> for CommentPageQuery {
    fn path(&self) -> String {
        String::from("/comments/")
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(anonymous) = self.anonymous {
            args.insert("anonymous", anonymous.to_string());
        }

        if let Some(ignore_users) = &self.ignore_users {
            args.insert("ignore_user", ignore_users.join(","));
        }

        if let Some(like) = &self.like {
            args.insert("like", like.to_owned());
        }

        if let Some(packages) = &self.packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(search) = &self.search {
            args.insert("search", search.to_owned());
        }

        if let Some(update_owners) = &self.update_owners {
            args.insert("update_owner", update_owners.join(","));
        }

        if let Some(updates) = &self.updates {
            args.insert("updates", updates.join(","));
        }

        if let Some(users) = &self.users {
            args.insert("user", users.join(","));
        }

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        Some(args)
    }

    fn parse(string: String) -> Result<CommentListPage, QueryError> {
        let comment_page: CommentListPage = serde_json::from_str(&string)?;
        Ok(comment_page)
    }

    fn missing() -> Result<CommentListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
