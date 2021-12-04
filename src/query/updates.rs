//! The contents of this module can be used to query a bodhi instance about existing updates.
//!
//! The [`UpdateIDQuery`](struct.UpdateIDQuery.html) returns exactly one
//! [`Update`](../../data/types/struct.Update.html), if and only if an `Update` with this ID or
//! alias exists - otherwise, it will return an error.
//!
//! The [`UpdateQuery`](struct.UpdateQuery.html) can be used to execute more complex queries, for
//! example filtering updates by release, status, security impact, reboot suggestion, or for updates
//! that are associated with a given set of packages.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::error::{QueryError, ServiceError};
use crate::service::DEFAULT_ROWS;
use crate::{BodhiService, Query, SinglePageQuery};

/// Use this for querying bodhi for a specific update by its ID or alias. It will either return an
/// `Ok(Some(Update))` matching the specified ID or alias, return `Ok(None)` if it doesn't exist, or
/// return an `Err(QueryError)` if another error occurred.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, UpdateIDQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// # #[cfg(feature = "online-tests")]
/// let update = bodhi.query(UpdateIDQuery::new("FEDORA-2019-3dd0cf468e")).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-0>
#[derive(Debug)]
pub struct UpdateIDQuery<'a> {
    id: &'a str,
}

#[derive(Debug, Deserialize)]
struct UpdatePage {
    update: Update,
    can_edit: bool,
}

impl<'a> UpdateIDQuery<'a> {
    /// This method is the only way to create a new `UpdateIDQuery` instance.
    pub fn new(id: &'a str) -> Self {
        UpdateIDQuery { id }
    }
}

impl<'a> SinglePageQuery<Option<Update>> for UpdateIDQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/updates/{}", self.id))
    }

    fn parse(string: &str) -> Result<Option<Update>, QueryError> {
        let update_page: UpdatePage = serde_json::from_str(string)?;
        Ok(Some(update_page.update))
    }

    fn missing() -> Result<Option<Update>, QueryError> {
        Ok(None)
    }
}

#[async_trait::async_trait]
impl<'a> Query<'a, Option<Update>> for UpdateIDQuery<'a> {
    async fn query(&'a self, bodhi: &'a BodhiService) -> Result<Option<Update>, QueryError> {
        <Self as SinglePageQuery<Option<Update>>>::query(self, bodhi)
    }
}

/// Use this for querying bodhi about a set of updates with the given properties, which can be
/// specified with the builder pattern. Note that some options can be specified multiple times, and
/// updates will be returned if any criteria match. This is consistent with both the web interface
/// and REST API behavior.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, FedoraRelease, UpdateRequest, UpdateQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// # #[cfg(feature = "online-tests")]
/// let updates = bodhi
///     .query(
///         UpdateQuery::new()
///             .users(vec!["decathorpe"])
///             .releases(vec![FedoraRelease::F30])
///             .request(UpdateRequest::Testing),
///     )
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-2>
#[derive(Default)]
pub struct UpdateQuery<'a> {
    active_releases: Option<bool>,
    aliases: Option<Vec<&'a str>>,
    approved_before: Option<&'a BodhiDate>,
    approved_since: Option<&'a BodhiDate>,
    bugs: Option<Vec<u32>>,
    builds: Option<Vec<&'a str>>,
    content_type: Option<ContentType>,
    critpath: Option<bool>,
    cves: Option<Vec<&'a str>>,
    like: Option<&'a str>,
    locked: Option<bool>,
    modified_before: Option<&'a BodhiDate>,
    modified_since: Option<&'a BodhiDate>,
    packages: Option<Vec<&'a str>>,
    pushed: Option<bool>,
    pushed_before: Option<&'a BodhiDate>,
    pushed_since: Option<&'a BodhiDate>,
    releases: Option<Vec<FedoraRelease>>,
    request: Option<UpdateRequest>,
    search: Option<&'a str>,
    severity: Option<UpdateSeverity>,
    status: Option<UpdateStatus>,
    submitted_before: Option<&'a BodhiDate>,
    submitted_since: Option<&'a BodhiDate>,
    suggest: Option<UpdateSuggestion>,
    update_ids: Option<Vec<&'a str>>,
    update_type: Option<UpdateType>,
    users: Option<Vec<&'a str>>,

    /// optional callback function for reporting progress
    callback: Option<Box<dyn FnMut(u32, u32) + 'a>>,
}

impl<'a> Debug for UpdateQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "UpdateQuery {{ active_releases: {:?}, aliases: {:?}, approved_before: {:?}, approved_since: {:?}, ",
            &self.active_releases, &self.aliases, &self.approved_before, &self.approved_since,
        )?;
        write!(
            f,
            "bugs: {:?}, builds: {:?}, content_type: {:?}, critpath: {:?}, cves: {:?}, like: {:?}, locked: {:?}, ",
            &self.bugs, &self.builds, &self.content_type, &self.critpath, &self.cves, &self.like, &self.locked,
        )?;
        write!(
            f,
            "modified_before: {:?}, modified_since: {:?}, packages: {:?}, pushed: {:?}, pushed_before: {:?}, ",
            &self.modified_before, &self.modified_since, &self.packages, &self.pushed, &self.pushed_before,
        )?;
        write!(
            f,
            "pushed_since: {:?}, releases: {:?}, request: {:?}, search: {:?}, severity: {:?}, status: {:?}, ",
            &self.pushed_since, &self.releases, &self.request, &self.search, &self.severity, &self.status,
        )?;
        write!(
            f,
            "submitted_before: {:?}, submitted_since: {:?}, suggest: {:?}, update_ids: {:?}, update_type: {:?}, ",
            &self.submitted_before, &self.submitted_since, &self.suggest, &self.update_ids, &self.update_type,
        )?;
        write!(f, "users: {:?} }}", &self.users)
    }
}

impl<'a> UpdateQuery<'a> {
    /// This method returns a new `UpdateQuery` with *no* filters set.
    pub fn new() -> Self {
        UpdateQuery {
            active_releases: None,
            aliases: None,
            approved_before: None,
            approved_since: None,
            bugs: None,
            builds: None,
            content_type: None,
            critpath: None,
            cves: None,
            like: None,
            locked: None,
            modified_before: None,
            modified_since: None,
            packages: None,
            pushed: None,
            pushed_before: None,
            pushed_since: None,
            releases: None,
            request: None,
            search: None,
            severity: None,
            status: None,
            submitted_before: None,
            submitted_since: None,
            suggest: None,
            update_ids: None,
            update_type: None,
            users: None,
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

    /// Restrict the returned results to (not) active releases.
    pub fn active_releases(mut self, active_releases: bool) -> Self {
        self.active_releases = Some(active_releases);
        self
    }

    /// Restrict results to updates matching the given alias(es).
    pub fn aliases(mut self, aliases: Vec<&'a str>) -> Self {
        self.aliases = Some(aliases);
        self
    }

    /// Restrict the returned results to updates which were approved
    /// before the given date and time.
    pub fn approved_before(mut self, approved_before: &'a BodhiDate) -> Self {
        self.approved_before = Some(approved_before);
        self
    }

    /// Restrict the returned results to updates which were approved
    /// since the given date and time.
    pub fn approved_since(mut self, approved_since: &'a BodhiDate) -> Self {
        self.approved_since = Some(approved_since);
        self
    }

    /// Restrict results to updates associated with the given bug(s).
    pub fn bugs(mut self, bugs: Vec<u32>) -> Self {
        self.bugs = Some(bugs);
        self
    }

    /// Restrict results to updates associated with the given build(s).
    pub fn builds(mut self, builds: Vec<&'a str>) -> Self {
        self.builds = Some(builds);
        self
    }

    /// Restrict the returned results to the given content type.
    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// Restrict the returned results to updates (not) marked with critpath.
    pub fn critpath(mut self, critpath: bool) -> Self {
        self.critpath = Some(critpath);
        self
    }

    /// Restrict results to updates associated with the given CVE(s).
    pub fn cves(mut self, cves: Vec<&'a str>) -> Self {
        self.cves = Some(cves);
        self
    }

    /// Restrict search to updates *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    /// Restrict the returned results to (not) locked updates.
    pub fn locked(mut self, locked: bool) -> Self {
        self.locked = Some(locked);
        self
    }

    /// Restrict the returned results to updates which were modified
    /// before the given date and time.
    pub fn modified_before(mut self, modified_before: &'a BodhiDate) -> Self {
        self.modified_before = Some(modified_before);
        self
    }

    /// Restrict the returned results to updates which were modified
    /// since the given date and time.
    pub fn modified_since(mut self, modified_since: &'a BodhiDate) -> Self {
        self.modified_since = Some(modified_since);
        self
    }

    /// Restrict results to updates associated for the given package(s).
    pub fn packages(mut self, packages: Vec<&'a str>) -> Self {
        self.packages = Some(packages);
        self
    }

    /// Restrict the returned results to (not) pushed updates.
    pub fn pushed(mut self, pushed: bool) -> Self {
        self.pushed = Some(pushed);
        self
    }

    /// Restrict the returned results to updates which were pushed
    /// before the given date and time.
    pub fn pushed_before(mut self, pushed_before: &'a BodhiDate) -> Self {
        self.pushed_before = Some(pushed_before);
        self
    }

    /// Restrict the returned results to updates which were pushed
    /// since the given date and time.
    pub fn pushed_since(mut self, pushed_since: &'a BodhiDate) -> Self {
        self.pushed_since = Some(pushed_since);
        self
    }

    /// Restrict results to updates for the given release(s).
    pub fn releases(mut self, releases: Vec<FedoraRelease>) -> Self {
        self.releases = Some(releases);
        self
    }

    /// Restrict the returned results to updates with the given request.
    pub fn request(mut self, request: UpdateRequest) -> Self {
        self.request = Some(request);
        self
    }

    /// Restrict search to updates containing the given argument.
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    /// Restrict the returned results to updates with the given severity.
    pub fn severity(mut self, severity: UpdateSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Restrict the returned results to updates with the given status.
    pub fn status(mut self, status: UpdateStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Restrict the returned results to updates which were submitted
    /// before the given date and time.
    pub fn submitted_before(mut self, submitted_before: &'a BodhiDate) -> Self {
        self.submitted_before = Some(submitted_before);
        self
    }

    /// Restrict the returned results to updates which were submitted
    /// since the given date and time.
    pub fn submitted_since(mut self, submitted_since: &'a BodhiDate) -> Self {
        self.submitted_since = Some(submitted_since);
        self
    }

    /// Restrict the returned results to updates with the given "suggest" value.
    pub fn suggest(mut self, suggest: UpdateSuggestion) -> Self {
        self.suggest = Some(suggest);
        self
    }

    /// Restrict results to updates matching the given update ID(s).
    pub fn update_ids(mut self, update_ids: Vec<&'a str>) -> Self {
        self.update_ids = Some(update_ids);
        self
    }

    /// Restrict results to updates matching the given update type.
    pub fn update_type(mut self, update_type: UpdateType) -> Self {
        self.update_type = Some(update_type);
        self
    }

    /// Restrict results to updates associated with the given user(s).
    pub fn users(mut self, users: Vec<&'a str>) -> Self {
        self.users = Some(users);
        self
    }

    /// Query the remote bodhi instance with the given parameters.
    async fn query(mut self, bodhi: &BodhiService) -> Result<Vec<Update>, QueryError> {
        let mut updates: Vec<Update> = Vec::new();
        let mut page = 1;

        // initial progress: 0 out of some
        if let Some(ref mut fun) = self.callback {
            fun(0, 1);
        }

        loop {
            let query = self.page_query(page, DEFAULT_ROWS);
            let result = query.query(bodhi).await?;

            if let Some(ref mut fun) = self.callback {
                fun(page, result.pages)
            }

            updates.extend(result.updates);
            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(updates)
    }

    fn page_query(&self, page: u32, rows_per_page: u32) -> UpdatePageQuery {
        UpdatePageQuery {
            active_releases: self.active_releases,
            aliases: self.aliases.as_ref(),
            approved_before: self.approved_before,
            approved_since: self.approved_since,
            bugs: self.bugs.as_ref(),
            builds: self.builds.as_ref(),
            content_type: self.content_type.as_ref(),
            critpath: self.critpath,
            cves: self.cves.as_ref(),
            like: self.like,
            locked: self.locked,
            modified_before: self.modified_before,
            modified_since: self.modified_since,
            packages: self.packages.as_ref(),
            pushed: self.pushed,
            pushed_before: self.pushed_before,
            pushed_since: self.pushed_since,
            releases: self.releases.as_ref(),
            request: self.request.as_ref(),
            search: self.search,
            severity: self.severity.as_ref(),
            status: self.status.as_ref(),
            submitted_before: self.submitted_before,
            submitted_since: self.submitted_since,
            suggest: self.suggest.as_ref(),
            update_ids: self.update_ids.as_ref(),
            update_type: self.update_type.as_ref(),
            users: self.users.as_ref(),
            page,
            rows_per_page,
        }
    }
}

#[async_trait::async_trait]
impl<'a> Query<'a, Vec<Update>> for UpdateQuery<'a> {
    async fn query(&'a self, bodhi: &'a BodhiService) -> Result<Vec<Update>, QueryError> {
        UpdateQuery::query(self, bodhi)
    }
}

#[derive(Debug, Deserialize)]
struct UpdateListPage {
    updates: Vec<Update>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

#[derive(Debug, Serialize)]
struct UpdatePageQuery<'a> {
    active_releases: Option<bool>,
    #[serde(rename = "alias")]
    aliases: Option<&'a Vec<&'a str>>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    approved_before: Option<&'a BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    approved_since: Option<&'a BodhiDate>,
    bugs: Option<&'a Vec<u32>>,
    builds: Option<&'a Vec<&'a str>>,
    content_type: Option<&'a ContentType>,
    critpath: Option<bool>,
    cves: Option<&'a Vec<&'a str>>,
    like: Option<&'a str>,
    locked: Option<bool>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    modified_before: Option<&'a BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    modified_since: Option<&'a BodhiDate>,
    packages: Option<&'a Vec<&'a str>>,
    pushed: Option<bool>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    pushed_before: Option<&'a BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    pushed_since: Option<&'a BodhiDate>,
    releases: Option<&'a Vec<FedoraRelease>>,
    request: Option<&'a UpdateRequest>,
    search: Option<&'a str>,
    severity: Option<&'a UpdateSeverity>,
    status: Option<&'a UpdateStatus>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    submitted_before: Option<&'a BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format_ref")]
    submitted_since: Option<&'a BodhiDate>,
    suggest: Option<&'a UpdateSuggestion>,
    #[serde(rename = "updateid")]
    update_ids: Option<&'a Vec<&'a str>>,
    #[serde(rename = "type")]
    update_type: Option<&'a UpdateType>,
    #[serde(rename = "user")]
    users: Option<&'a Vec<&'a str>>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> SinglePageQuery<UpdateListPage> for UpdatePageQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/updates/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(string: &str) -> Result<UpdateListPage, QueryError> {
        let update_page: UpdateListPage = serde_json::from_str(string)?;
        Ok(update_page)
    }

    fn missing() -> Result<UpdateListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
