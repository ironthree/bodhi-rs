use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::client::DEFAULT_ROWS;
use crate::data::*;
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

/// data type encapsulating parameters for querying for a [`Update`] by alias
///
/// If no comment with the specified ID is known to bodhi, a [`QueryError::NotFound`] error is
/// returned for the query.
///
/// ```
/// use bodhi::UpdateIDQuery;
///
/// let query = UpdateIDQuery::new("FEDORA-2019-3dd0cf468e");
/// // let update = bodhi.request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-0>
#[derive(Debug)]
pub struct UpdateIDQuery<'a> {
    id: &'a str,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UpdatePage {
    update: Update,
    can_edit: bool,
}

impl<'a> UpdateIDQuery<'a> {
    /// constructor for [`UpdateIDQuery`] from a comment ID
    pub fn new(id: &'a str) -> Self {
        UpdateIDQuery { id }
    }
}

impl<'a> SingleRequest<UpdatePage, Update> for UpdateIDQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/updates/{}", self.id))
    }

    fn parse(&self, string: &str) -> Result<UpdatePage, QueryError> {
        let page: UpdatePage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: UpdatePage) -> Update {
        page.update
    }
}


/// data type encapsulating parameters for querying [`Update`]s
///
/// ```
/// use bodhi::{ContentType, FedoraRelease, UpdateQuery, UpdateRequest};
///
/// let query = UpdateQuery::new()
///     .users(&["decathorpe"])
///     .releases(&[&FedoraRelease::fedora(34, ContentType::RPM).unwrap()])
///     .request(UpdateRequest::Testing);
/// // let updates = bodhi.paginated_request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-2>
#[derive(Default)]
pub struct UpdateQuery<'a> {
    active_releases: Option<bool>,
    aliases: Option<&'a [&'a str]>,
    approved_before: Option<&'a BodhiDate>,
    approved_since: Option<&'a BodhiDate>,
    bugs: Option<&'a [u32]>,
    builds: Option<&'a [&'a str]>,
    content_type: Option<ContentType>,
    critpath: Option<bool>,
    cves: Option<&'a [&'a str]>,
    like: Option<&'a str>,
    locked: Option<bool>,
    modified_before: Option<&'a BodhiDate>,
    modified_since: Option<&'a BodhiDate>,
    packages: Option<&'a [&'a str]>,
    pushed: Option<bool>,
    pushed_before: Option<&'a BodhiDate>,
    pushed_since: Option<&'a BodhiDate>,
    releases: Option<&'a [&'a FedoraRelease]>,
    request: Option<UpdateRequest>,
    search: Option<&'a str>,
    severity: Option<UpdateSeverity>,
    status: Option<UpdateStatus>,
    submitted_before: Option<&'a BodhiDate>,
    submitted_since: Option<&'a BodhiDate>,
    suggest: Option<UpdateSuggestion>,
    update_ids: Option<&'a [&'a str]>,
    update_type: Option<UpdateType>,
    users: Option<&'a [&'a str]>,

    // number of results per page
    rows_per_page: u32,
    // optional callback function for reporting progress
    callback: Option<Box<dyn Fn(u32, u32) + 'a>>,
}

impl<'a> Debug for UpdateQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateQuery")
            .field("active_releases", &self.active_releases)
            .field("aliases", &self.aliases)
            .field("approved_before", &self.approved_before)
            .field("approved_since", &self.approved_since)
            .field("bugs", &self.bugs)
            .field("builds", &self.builds)
            .field("content_type", &self.content_type)
            .field("critpath", &self.critpath)
            .field("cves", &self.cves)
            .field("like", &self.like)
            .field("locked", &self.locked)
            .field("modified_before", &self.modified_before)
            .field("modified_since", &self.modified_since)
            .field("packages", &self.packages)
            .field("pushed", &self.pushed)
            .field("pushed_before", &self.pushed_before)
            .field("pushed_since", &self.pushed_since)
            .field("releases", &self.releases)
            .field("request", &self.request)
            .field("search", &self.search)
            .field("severity", &self.severity)
            .field("status", &self.status)
            .field("submitted_before", &self.submitted_before)
            .field("submitted_since", &self.submitted_since)
            .field("suggest", &self.suggest)
            .field("update_ids", &self.update_ids)
            .field("update_type", &self.update_type)
            .field("users", &self.users)
            .field("rows_per_page", &self.rows_per_page)
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> UpdateQuery<'a> {
    /// constructor for [`UpdateQuery`] without any filters
    pub fn new() -> Self {
        UpdateQuery {
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

    /// restrict query to updates from (in)active releases
    #[must_use]
    pub fn active_releases(mut self, active_releases: bool) -> Self {
        self.active_releases = Some(active_releases);
        self
    }

    /// restrict query to updates matching the specified aliases
    #[must_use]
    pub fn aliases(mut self, aliases: &'a [&'a str]) -> Self {
        self.aliases = Some(aliases);
        self
    }

    /// restrict query to updates that have been approved before the specified date & time
    #[deprecated(
        since = "2.0.0",
        note = "`date_approved` is an unused field: <https://github.com/fedora-infra/bodhi/issues/4171>"
    )]
    #[must_use]
    pub fn approved_before(mut self, approved_before: &'a BodhiDate) -> Self {
        self.approved_before = Some(approved_before);
        self
    }

    /// restrict query to updates that have been approved since the specified date & time
    #[deprecated(
        since = "2.0.0",
        note = "`date_approved` is an unused field: <https://github.com/fedora-infra/bodhi/issues/4171>"
    )]
    #[must_use]
    pub fn approved_since(mut self, approved_since: &'a BodhiDate) -> Self {
        self.approved_since = Some(approved_since);
        self
    }

    /// restrict query to updates that are associated with any of the specified bugs
    #[must_use]
    pub fn bugs(mut self, bugs: &'a [u32]) -> Self {
        self.bugs = Some(bugs);
        self
    }

    /// restrict query to updates that are associated with any of the specified builds
    #[must_use]
    pub fn builds(mut self, builds: &'a [&'a str]) -> Self {
        self.builds = Some(builds);
        self
    }

    /// restrict query to updates of the given content type
    #[must_use]
    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// restrict query to updates that do (not) contain packages in the "critical path"
    #[must_use]
    pub fn critpath(mut self, critpath: bool) -> Self {
        self.critpath = Some(critpath);
        self
    }

    /// restrict query to updates that are associated with any of the specified CVEs
    #[must_use]
    pub fn cves(mut self, cves: &'a [&'a str]) -> Self {
        self.cves = Some(cves);
        self
    }

    /// restrict query to updates where the text is "like" the given string (in the SQL sense)
    #[must_use]
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    /// restrict query to updates that are (not) locked
    #[must_use]
    pub fn locked(mut self, locked: bool) -> Self {
        self.locked = Some(locked);
        self
    }

    /// restrict query to updates that have been modified before the specified date & time
    #[must_use]
    pub fn modified_before(mut self, modified_before: &'a BodhiDate) -> Self {
        self.modified_before = Some(modified_before);
        self
    }

    /// restrict query to updates that have been modified since the specified date & time
    #[must_use]
    pub fn modified_since(mut self, modified_since: &'a BodhiDate) -> Self {
        self.modified_since = Some(modified_since);
        self
    }

    /// restrict query to updates that contain any of the specified packages
    #[must_use]
    pub fn packages(mut self, packages: &'a [&'a str]) -> Self {
        self.packages = Some(packages);
        self
    }

    /// restrict query to updates that have (not) been pushed
    #[must_use]
    pub fn pushed(mut self, pushed: bool) -> Self {
        self.pushed = Some(pushed);
        self
    }

    /// restrict query to updates that have been pushed before the specified date & time
    #[must_use]
    pub fn pushed_before(mut self, pushed_before: &'a BodhiDate) -> Self {
        self.pushed_before = Some(pushed_before);
        self
    }

    /// restrict query to updates that have been pushed since the specified date & time
    #[must_use]
    pub fn pushed_since(mut self, pushed_since: &'a BodhiDate) -> Self {
        self.pushed_since = Some(pushed_since);
        self
    }

    /// restrict query to updates for any of the specified releases
    #[must_use]
    pub fn releases(mut self, releases: &'a [&'a FedoraRelease]) -> Self {
        self.releases = Some(releases);
        self
    }

    /// restrict query to updates that have been requested for another state
    #[must_use]
    pub fn request(mut self, request: UpdateRequest) -> Self {
        self.request = Some(request);
        self
    }

    /// restrict query to updates matching a search keyword
    #[must_use]
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    /// restrict query to updates with the specified severity
    #[must_use]
    pub fn severity(mut self, severity: UpdateSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// restrict query to updates with the specified status
    #[must_use]
    pub fn status(mut self, status: UpdateStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// restrict query to updates that have been submitted before the specified date & time
    #[must_use]
    pub fn submitted_before(mut self, submitted_before: &'a BodhiDate) -> Self {
        self.submitted_before = Some(submitted_before);
        self
    }

    /// restrict query to updates that have been submitted since the specified date & time
    #[must_use]
    pub fn submitted_since(mut self, submitted_since: &'a BodhiDate) -> Self {
        self.submitted_since = Some(submitted_since);
        self
    }

    /// restrict query to updates with the specified suggested action
    #[must_use]
    pub fn suggest(mut self, suggest: UpdateSuggestion) -> Self {
        self.suggest = Some(suggest);
        self
    }

    /// restrict query to updates matching any of the specified update IDs
    #[must_use]
    pub fn update_ids(mut self, update_ids: &'a [&'a str]) -> Self {
        self.update_ids = Some(update_ids);
        self
    }

    /// restrict query to updates with the specified update type
    #[must_use]
    pub fn update_type(mut self, update_type: UpdateType) -> Self {
        self.update_type = Some(update_type);
        self
    }

    /// restrict query to updates that have been submitted by any of the specified users
    #[must_use]
    pub fn users(mut self, users: &'a [&'a str]) -> Self {
        self.users = Some(users);
        self
    }
}


/// data type encapsulating parameters for querying specific [`UpdateQuery`] result pages
#[derive(Debug, Serialize)]
pub struct UpdatePageQuery<'a> {
    active_releases: Option<bool>,
    #[serde(rename = "alias")]
    aliases: Option<&'a [&'a str]>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    approved_before: Option<&'a BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    approved_since: Option<&'a BodhiDate>,
    bugs: Option<&'a [u32]>,
    builds: Option<&'a [&'a str]>,
    content_type: Option<ContentType>,
    critpath: Option<bool>,
    cves: Option<&'a [&'a str]>,
    like: Option<&'a str>,
    locked: Option<bool>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    modified_before: Option<&'a BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    modified_since: Option<&'a BodhiDate>,
    packages: Option<&'a [&'a str]>,
    pushed: Option<bool>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    pushed_before: Option<&'a BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    pushed_since: Option<&'a BodhiDate>,
    releases: Option<&'a [&'a FedoraRelease]>,
    request: Option<UpdateRequest>,
    search: Option<&'a str>,
    severity: Option<UpdateSeverity>,
    status: Option<UpdateStatus>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    submitted_before: Option<&'a BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    submitted_since: Option<&'a BodhiDate>,
    suggest: Option<UpdateSuggestion>,
    #[serde(rename = "updateid")]
    update_ids: Option<&'a [&'a str]>,
    #[serde(rename = "type")]
    update_type: Option<UpdateType>,
    #[serde(rename = "user")]
    users: Option<&'a [&'a str]>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> UpdatePageQuery<'a> {
    /// constructor for [`UpdatePageQuery`] taking parameters from an existing [`UpdateQuery`]
    pub fn from_query(query: &'a UpdateQuery, page: u32) -> Self {
        UpdatePageQuery {
            active_releases: query.active_releases,
            aliases: query.aliases,
            approved_before: query.approved_before,
            approved_since: query.approved_since,
            bugs: query.bugs,
            builds: query.builds,
            content_type: query.content_type,
            critpath: query.critpath,
            cves: query.cves,
            like: query.like,
            locked: query.locked,
            modified_before: query.modified_before,
            modified_since: query.modified_since,
            packages: query.packages,
            pushed: query.pushed,
            pushed_before: query.pushed_before,
            pushed_since: query.pushed_since,
            releases: query.releases,
            request: query.request,
            search: query.search,
            severity: query.severity,
            status: query.status,
            submitted_before: query.submitted_before,
            submitted_since: query.submitted_since,
            suggest: query.suggest,
            update_ids: query.update_ids,
            update_type: query.update_type,
            users: query.users,
            page,
            rows_per_page: query.rows_per_page,
        }
    }
}

impl<'a> SingleRequest<UpdateListPage, Vec<Update>> for UpdatePageQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/updates/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(&self, string: &str) -> Result<UpdateListPage, QueryError> {
        let page: UpdateListPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: UpdateListPage) -> Vec<Update> {
        page.updates
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UpdateListPage {
    updates: Vec<Update>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

impl Pagination for UpdateListPage {
    fn pages(&self) -> u32 {
        self.pages
    }
}

impl<'a> PaginatedRequest<UpdateListPage, Vec<Update>> for UpdateQuery<'a> {
    fn page_request<'b>(&'b self, page: u32) -> Box<dyn SingleRequest<UpdateListPage, Vec<Update>> + 'b> {
        Box::new(UpdatePageQuery::from_query(self, page))
    }

    fn callback(&self, page: u32, pages: u32) {
        if let Some(ref callback) = &self.callback {
            callback(page, pages)
        }
    }
}
