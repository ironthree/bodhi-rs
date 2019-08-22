//! query comments (or *one* comment by ID)
//!
//! The contents of this module can be used to query a bodhi instance about
//! existing comments.
//!
//! The `CommentIDQuery` returns exactly one Comment, if and only if a
//! Comment with the given integer ID exists - otherwise, it will return an
//! error.
//!
//! The `CommentQuery` can be used to execute more complex queries, for example
//! filtering comments that are associated with a set of updates or packages,
//! or query comments made by certain users, or filed against updates that were
//! created by specific users.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::Comment;
use crate::error::{BodhiError, QueryError};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

use super::retry_query;

/// Use this for querying bodhi for a specific comment by its ID.
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::new(String::from(bodhi::FEDORA_BODHI_URL))
///     .build().unwrap();
///
/// let comment = bodhi::query::CommentIDQuery::new(19999).query(&bodhi).unwrap();
/// ```
#[derive(Debug)]
pub struct CommentIDQuery {
    id: u32,
}

#[derive(Debug, Deserialize)]
struct CommentPage {
    comment: Comment,
}

impl CommentIDQuery {
    /// This method is the only way to create a new `CommentIDQuery` instance.
    pub fn new(id: u32) -> Self {
        CommentIDQuery { id }
    }

    /// This method will query the remote bodhi instance for the requested comment by ID,
    /// and will either return an `Ok(Some(Comment))` matching the specified ID,
    /// return `Ok(None)` if it doesn't exist, or return an `Err(String)`
    /// if another error occurred.
    pub fn query(self, bodhi: &BodhiService) -> Result<Option<Comment>, QueryError> {
        let path = format!("/comments/{}", self.id);

        let mut response = bodhi.get(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let result = response.text()?;
            let comment: CommentPage = serde_json::from_str(&result)?;

            Ok(Some(comment.comment))
        } else if status == 404 {
            // bodhi query successful, but comment not found
            Ok(None)
        } else {
            // other server-side error
            let result = response.text()?;
            let error: BodhiError = serde_json::from_str(&result)?;

            Err(QueryError::BodhiError { error })
        }
    }
}

/// Use this for querying bodhi about a set of comments with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and comments will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::new(String::from(bodhi::FEDORA_BODHI_URL))
///     .build().unwrap();
///
/// let comments = bodhi::query::CommentQuery::new()
///     .anonymous(true)
///     .users(String::from("decathorpe"))
///     .packages(String::from("rust"))
///     .query(&bodhi).unwrap();
/// ```
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
    /// This method returns a new `CommentQuery` with *no* filters set.
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
    pub fn anonymous(mut self, anonymous: bool) -> Self {
        self.anonymous = Some(anonymous);
        self
    }

    /// Restrict results to ignore comments by certain users.
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

    /// Restrict the returned results to comments filed against updates for the
    /// given package(s). Can be specified multiple times.
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

    /// Restrict the returned results to comments filed against updates
    /// created by the specified user(s). Can be specified multiple times.
    pub fn update_owners(mut self, update_owner: String) -> Self {
        match &mut self.update_owners {
            Some(update_owners) => update_owners.push(update_owner),
            None => self.update_owners = Some(vec![update_owner]),
        }

        self
    }

    /// Restrict the returned results to comments filed against the given update(s).
    /// Can be specified multiple times.
    pub fn updates(mut self, update: String) -> Self {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    /// Restrict the returned results to comments filed by the given user(s).
    /// Can be specified multiple times.
    pub fn users(mut self, user: String) -> Self {
        match &mut self.users {
            Some(users) => users.push(user),
            None => self.users = Some(vec![user]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Comment>, QueryError> {
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

    fn query(self, bodhi: &BodhiService) -> Result<CommentListPage, QueryError> {
        let path = String::from("/comments/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(anonymous) = self.anonymous {
            args.insert("anonymous", anonymous.to_string());
        }

        if let Some(ignore_users) = self.ignore_users {
            args.insert("ignore_user", ignore_users.join(","));
        }

        if let Some(like) = self.like {
            args.insert("like", like);
        }

        if let Some(packages) = self.packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(search) = self.search {
            args.insert("search", search);
        }

        if let Some(update_owners) = self.update_owners {
            args.insert("update_owner", update_owners.join(","));
        }

        if let Some(updates) = self.updates {
            args.insert("updates", updates.join(","));
        }

        if let Some(users) = self.users {
            args.insert("user", users.join(","));
        }

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        let result = retry_query(bodhi, &path, args)?;
        let comments: CommentListPage = serde_json::from_str(&result)?;

        Ok(comments)
    }
}
