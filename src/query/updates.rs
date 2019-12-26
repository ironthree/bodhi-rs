//! The contents of this module can be used to query a bodhi instance about existing updates.
//!
//! The `UpdateIDQuery` returns exactly one Update, if and only if a Update with this ID or alias
//! exists - otherwise, it will return an error.
//!
//! The `UpdateQuery` can be used to execute more complex queries, for example filtering updates by
//! release, status, security impact, reboot suggestion, or for updates that are associated with a
//! given set of packages.

use serde::{Deserialize, Serialize};

use crate::data::{
    ContentType,
    FedoraRelease,
    Update,
    UpdateRequest,
    UpdateSeverity,
    UpdateStatus,
    UpdateSuggestion,
    UpdateType,
};
use crate::error::{QueryError, ServiceError};
use crate::query::{Query, SinglePageQuery};
use crate::service::{BodhiService, DEFAULT_ROWS};

/// Use this for querying bodhi for a specific update by its ID or alias. It will either return an
/// `Ok(Some(Update))` matching the specified ID or alias, return `Ok(None)` if it doesn't exist, or
/// return an `Err(String)` if another error occurred.
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::default().build().unwrap();
///
/// let update = bodhi
///     .query(&bodhi::query::UpdateIDQuery::new(String::from(
///         "FEDORA-2019-3dd0cf468e",
///     )))
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-0>
#[derive(Debug)]
pub struct UpdateIDQuery {
    id: String,
}

#[derive(Debug, Deserialize)]
struct UpdatePage {
    update: Update,
    can_edit: bool,
}

impl UpdateIDQuery {
    /// This method is the only way to create a new `UpdateIDQuery` instance.
    pub fn new(id: String) -> Self {
        UpdateIDQuery { id }
    }
}

impl SinglePageQuery<Option<Update>> for UpdateIDQuery {
    fn path(&self) -> String {
        format!("/updates/{}", self.id)
    }

    fn parse(string: String) -> Result<Option<Update>, QueryError> {
        let update_page: UpdatePage = serde_json::from_str(&string)?;
        Ok(Some(update_page.update))
    }

    fn missing() -> Result<Option<Update>, QueryError> {
        Ok(None)
    }
}

impl Query<Option<Update>> for UpdateIDQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Option<Update>, QueryError> {
        <Self as SinglePageQuery<Option<Update>>>::query(self, bodhi)
    }
}

/// Use this for querying bodhi about a set of updates with the given properties, which can be
/// specified with the builder pattern. Note that some options can be specified multiple times, and
/// updates will be returned if any criteria match. This is consistent with both the web interface
/// and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::default().build().unwrap();
///
/// let updates = bodhi
///     .query(
///         &bodhi::query::UpdateQuery::new()
///             .users(String::from("decathorpe"))
///             .releases(bodhi::data::FedoraRelease::F30)
///             .status(bodhi::data::UpdateStatus::Testing),
///     )
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-2>
#[derive(Debug, Default)]
pub struct UpdateQuery {
    active_releases: Option<bool>,
    aliases: Option<Vec<String>>,
    approved_before: Option<String>,
    approved_since: Option<String>,
    bugs: Option<Vec<String>>,
    builds: Option<Vec<String>>,
    content_type: Option<ContentType>,
    critpath: Option<bool>,
    cves: Option<Vec<String>>,
    like: Option<String>,
    locked: Option<bool>,
    modified_before: Option<String>,
    modified_since: Option<String>,
    packages: Option<Vec<String>>,
    pushed: Option<bool>,
    pushed_before: Option<String>,
    pushed_since: Option<String>,
    releases: Option<Vec<FedoraRelease>>,
    request: Option<UpdateRequest>,
    search: Option<String>,
    severity: Option<UpdateSeverity>,
    status: Option<UpdateStatus>,
    submitted_before: Option<String>,
    submitted_since: Option<String>,
    suggest: Option<UpdateSuggestion>,
    update_ids: Option<Vec<String>>,
    update_type: Option<UpdateType>,
    users: Option<Vec<String>>,
}

impl UpdateQuery {
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
        }
    }

    /// Restrict the returned results to (not) active releases.
    pub fn active_releases(mut self, active_releases: bool) -> Self {
        self.active_releases = Some(active_releases);
        self
    }

    /// Restrict results to updates matching the given alias(es).
    ///
    /// Can be specified multiple times.
    pub fn aliases(mut self, alias: String) -> Self {
        match &mut self.aliases {
            Some(aliases) => aliases.push(alias),
            None => self.aliases = Some(vec![alias]),
        }

        self
    }

    /// Restrict the returned results to updates which were approved
    /// before the given date and time.
    pub fn approved_before(mut self, approved_before: String) -> Self {
        self.approved_before = Some(approved_before);
        self
    }

    /// Restrict the returned results to updates which were approved
    /// since the given date and time.
    pub fn approved_since(mut self, approved_since: String) -> Self {
        self.approved_since = Some(approved_since);
        self
    }

    /// Restrict results to updates associated with the given bug(s).
    ///
    /// Can be specified multiple times.
    pub fn bugs(mut self, bug: String) -> Self {
        match &mut self.bugs {
            Some(bugs) => bugs.push(bug),
            None => self.bugs = Some(vec![bug]),
        }

        self
    }

    /// Restrict results to updates associated with the given build(s).
    ///
    /// Can be specified multiple times.
    pub fn builds(mut self, build: String) -> Self {
        match &mut self.builds {
            Some(builds) => builds.push(build),
            None => self.builds = Some(vec![build]),
        }

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
    ///
    /// Can be specified multiple times.
    pub fn cves(mut self, cve: String) -> Self {
        match &mut self.cves {
            Some(cves) => cves.push(cve),
            None => self.cves = Some(vec![cve]),
        }

        self
    }

    /// Restrict search to updates *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: String) -> Self {
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
    pub fn modified_before(mut self, modified_before: String) -> Self {
        self.modified_before = Some(modified_before);
        self
    }

    /// Restrict the returned results to updates which were modified
    /// since the given date and time.
    pub fn modified_since(mut self, modified_since: String) -> Self {
        self.modified_since = Some(modified_since);
        self
    }

    /// Restrict results to updates associated for the given package(s).
    ///
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> Self {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict the returned results to (not) pushed updates.
    pub fn pushed(mut self, pushed: bool) -> Self {
        self.pushed = Some(pushed);
        self
    }

    /// Restrict the returned results to updates which were pushed
    /// before the given date and time.
    pub fn pushed_before(mut self, pushed_before: String) -> Self {
        self.pushed_before = Some(pushed_before);
        self
    }

    /// Restrict the returned results to updates which were pushed
    /// since the given date and time.
    pub fn pushed_since(mut self, pushed_since: String) -> Self {
        self.pushed_since = Some(pushed_since);
        self
    }

    /// Restrict results to updates for the given release(s).
    ///
    /// Can be specified multiple times.
    pub fn releases(mut self, release: FedoraRelease) -> Self {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec![release]),
        }

        self
    }

    /// Restrict the returned results to updates with the given request.
    pub fn request(mut self, request: UpdateRequest) -> Self {
        self.request = Some(request);
        self
    }

    /// Restrict search to updates containing the given argument.
    pub fn search(mut self, search: String) -> Self {
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
    pub fn submitted_before(mut self, submitted_before: String) -> Self {
        self.submitted_before = Some(submitted_before);
        self
    }

    /// Restrict the returned results to updates which were submitted
    /// since the given date and time.
    pub fn submitted_since(mut self, submitted_since: String) -> Self {
        self.submitted_since = Some(submitted_since);
        self
    }

    /// Restrict the returned results to updates with the given "suggest" value.
    pub fn suggest(mut self, suggest: UpdateSuggestion) -> Self {
        self.suggest = Some(suggest);
        self
    }

    /// Restrict results to updates matching the given update ID(s).
    ///
    /// Can be specified multiple times.
    pub fn update_ids(mut self, update_id: String) -> Self {
        match &mut self.update_ids {
            Some(update_ids) => update_ids.push(update_id),
            None => self.update_ids = Some(vec![update_id]),
        }

        self
    }

    /// Restrict results to updates matching the given update type.
    pub fn update_type(mut self, update_type: UpdateType) -> Self {
        self.update_type = Some(update_type);
        self
    }

    /// Restrict results to updates associated with the given user(s).
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
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Update>, QueryError> {
        let mut updates: Vec<Update> = Vec::new();
        let mut page = 1;

        loop {
            let query = self.page_query(page, DEFAULT_ROWS);
            let result = query.query(bodhi)?;

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
            approved_before: self.approved_before.as_ref(),
            approved_since: self.approved_since.as_ref(),
            bugs: self.bugs.as_ref(),
            builds: self.builds.as_ref(),
            content_type: self.content_type.as_ref(),
            critpath: self.critpath,
            cves: self.cves.as_ref(),
            like: self.like.as_ref(),
            locked: self.locked,
            modified_before: self.modified_before.as_ref(),
            modified_since: self.modified_since.as_ref(),
            packages: self.packages.as_ref(),
            pushed: self.pushed,
            pushed_before: self.pushed_before.as_ref(),
            pushed_since: self.pushed_since.as_ref(),
            releases: self.releases.as_ref(),
            request: self.request.as_ref(),
            search: self.search.as_ref(),
            severity: self.severity.as_ref(),
            status: self.status.as_ref(),
            submitted_before: self.submitted_before.as_ref(),
            submitted_since: self.submitted_since.as_ref(),
            suggest: self.suggest.as_ref(),
            update_ids: self.update_ids.as_ref(),
            update_type: self.update_type.as_ref(),
            users: self.users.as_ref(),
            page,
            rows_per_page,
        }
    }
}

impl Query<Vec<Update>> for UpdateQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Update>, QueryError> {
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
    aliases: Option<&'a Vec<String>>,
    approved_before: Option<&'a String>,
    approved_since: Option<&'a String>,
    bugs: Option<&'a Vec<String>>,
    builds: Option<&'a Vec<String>>,
    content_type: Option<&'a ContentType>,
    critpath: Option<bool>,
    cves: Option<&'a Vec<String>>,
    like: Option<&'a String>,
    locked: Option<bool>,
    modified_before: Option<&'a String>,
    modified_since: Option<&'a String>,
    packages: Option<&'a Vec<String>>,
    pushed: Option<bool>,
    pushed_before: Option<&'a String>,
    pushed_since: Option<&'a String>,
    releases: Option<&'a Vec<FedoraRelease>>,
    request: Option<&'a UpdateRequest>,
    search: Option<&'a String>,
    severity: Option<&'a UpdateSeverity>,
    status: Option<&'a UpdateStatus>,
    submitted_before: Option<&'a String>,
    submitted_since: Option<&'a String>,
    suggest: Option<&'a UpdateSuggestion>,
    update_ids: Option<&'a Vec<String>>,
    update_type: Option<&'a UpdateType>,
    users: Option<&'a Vec<String>>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> SinglePageQuery<UpdateListPage> for UpdatePageQuery<'a> {
    fn path(&self) -> String {
        format!("/updates/?{}", serde_url_params::to_string(self).unwrap())
    }

    fn parse(string: String) -> Result<UpdateListPage, QueryError> {
        let update_page: UpdateListPage = serde_json::from_str(&string)?;
        Ok(update_page)
    }

    fn missing() -> Result<UpdateListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
