// ! The contents of this module can be used to query a bodhi instance about existing updates.
// !
// ! The [`UpdateIDQuery`](struct.UpdateIDQuery.html) returns exactly one
// ! [`Update`](../../data/types/struct.Update.html), if and only if an `Update` with this ID or
// ! alias exists - otherwise, it will return an error.
// !
// ! The [`UpdateQuery`](struct.UpdateQuery.html) can be used to execute more complex queries, for
// ! example filtering updates by release, status, security impact, reboot suggestion, or for
// updates ! that are associated with a given set of packages.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};
use crate::service::DEFAULT_ROWS;

// Use this for querying bodhi for a specific update by its ID or alias. It will either return an
// `Ok(Some(Update))` matching the specified ID or alias, return `Ok(None)` if it doesn't exist, or
// return an `Err(QueryError)` if another error occurred.
//
// ```
// # use bodhi::{BodhiServiceBuilder, UpdateIDQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let update = bodhi.query(UpdateIDQuery::new("FEDORA-2019-3dd0cf468e")).unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-0>
#[derive(Debug)]
pub struct UpdateIDQuery<'a> {
    id: &'a str,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpdatePage {
    update: Update,
    can_edit: bool,
}

impl<'a> UpdateIDQuery<'a> {
    // This method is the only way to create a new `UpdateIDQuery` instance.
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

    fn body(&self) -> Option<String> {
        None
    }

    fn parse(&self, string: &str) -> Result<UpdatePage, QueryError> {
        let page: UpdatePage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: UpdatePage) -> Update {
        page.update
    }
}

// Use this for querying bodhi about a set of updates with the given properties, which can be
// specified with the builder pattern. Note that some options can be specified multiple times, and
// updates will be returned if any criteria match. This is consistent with both the web interface
// and REST API behavior.
//
// ```
// # use bodhi::{BodhiServiceBuilder, FedoraRelease, UpdateRequest, UpdateQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let updates = bodhi
//     .query(
//         UpdateQuery::new()
//             .users(vec!["decathorpe"])
//             .releases(vec![FedoraRelease::F30])
//             .request(UpdateRequest::Testing),
//     )
//     .unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-2>
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
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> UpdateQuery<'a> {
    // This method returns a new `UpdateQuery` with *no* filters set.
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

    // Restrict the returned results to (not) active releases.
    pub fn active_releases(mut self, active_releases: bool) -> Self {
        self.active_releases = Some(active_releases);
        self
    }

    // Restrict results to updates matching the given alias(es).
    pub fn aliases(mut self, aliases: Vec<&'a str>) -> Self {
        self.aliases = Some(aliases);
        self
    }

    // Restrict the returned results to updates which were approved
    // before the given date and time.
    pub fn approved_before(mut self, approved_before: &'a BodhiDate) -> Self {
        self.approved_before = Some(approved_before);
        self
    }

    // Restrict the returned results to updates which were approved
    // since the given date and time.
    pub fn approved_since(mut self, approved_since: &'a BodhiDate) -> Self {
        self.approved_since = Some(approved_since);
        self
    }

    // Restrict results to updates associated with the given bug(s).
    pub fn bugs(mut self, bugs: Vec<u32>) -> Self {
        self.bugs = Some(bugs);
        self
    }

    // Restrict results to updates associated with the given build(s).
    pub fn builds(mut self, builds: Vec<&'a str>) -> Self {
        self.builds = Some(builds);
        self
    }

    // Restrict the returned results to the given content type.
    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self
    }

    // Restrict the returned results to updates (not) marked with critpath.
    pub fn critpath(mut self, critpath: bool) -> Self {
        self.critpath = Some(critpath);
        self
    }

    // Restrict results to updates associated with the given CVE(s).
    pub fn cves(mut self, cves: Vec<&'a str>) -> Self {
        self.cves = Some(cves);
        self
    }

    // Restrict search to updates *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    // Restrict the returned results to (not) locked updates.
    pub fn locked(mut self, locked: bool) -> Self {
        self.locked = Some(locked);
        self
    }

    // Restrict the returned results to updates which were modified
    // before the given date and time.
    pub fn modified_before(mut self, modified_before: &'a BodhiDate) -> Self {
        self.modified_before = Some(modified_before);
        self
    }

    // Restrict the returned results to updates which were modified
    // since the given date and time.
    pub fn modified_since(mut self, modified_since: &'a BodhiDate) -> Self {
        self.modified_since = Some(modified_since);
        self
    }

    // Restrict results to updates associated for the given package(s).
    pub fn packages(mut self, packages: Vec<&'a str>) -> Self {
        self.packages = Some(packages);
        self
    }

    // Restrict the returned results to (not) pushed updates.
    pub fn pushed(mut self, pushed: bool) -> Self {
        self.pushed = Some(pushed);
        self
    }

    // Restrict the returned results to updates which were pushed
    // before the given date and time.
    pub fn pushed_before(mut self, pushed_before: &'a BodhiDate) -> Self {
        self.pushed_before = Some(pushed_before);
        self
    }

    // Restrict the returned results to updates which were pushed
    // since the given date and time.
    pub fn pushed_since(mut self, pushed_since: &'a BodhiDate) -> Self {
        self.pushed_since = Some(pushed_since);
        self
    }

    // Restrict results to updates for the given release(s).
    pub fn releases(mut self, releases: Vec<FedoraRelease>) -> Self {
        self.releases = Some(releases);
        self
    }

    // Restrict the returned results to updates with the given request.
    pub fn request(mut self, request: UpdateRequest) -> Self {
        self.request = Some(request);
        self
    }

    // Restrict search to updates containing the given argument.
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    // Restrict the returned results to updates with the given severity.
    pub fn severity(mut self, severity: UpdateSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    // Restrict the returned results to updates with the given status.
    pub fn status(mut self, status: UpdateStatus) -> Self {
        self.status = Some(status);
        self
    }

    // Restrict the returned results to updates which were submitted
    // before the given date and time.
    pub fn submitted_before(mut self, submitted_before: &'a BodhiDate) -> Self {
        self.submitted_before = Some(submitted_before);
        self
    }

    // Restrict the returned results to updates which were submitted
    // since the given date and time.
    pub fn submitted_since(mut self, submitted_since: &'a BodhiDate) -> Self {
        self.submitted_since = Some(submitted_since);
        self
    }

    // Restrict the returned results to updates with the given "suggest" value.
    pub fn suggest(mut self, suggest: UpdateSuggestion) -> Self {
        self.suggest = Some(suggest);
        self
    }

    // Restrict results to updates matching the given update ID(s).
    pub fn update_ids(mut self, update_ids: Vec<&'a str>) -> Self {
        self.update_ids = Some(update_ids);
        self
    }

    // Restrict results to updates matching the given update type.
    pub fn update_type(mut self, update_type: UpdateType) -> Self {
        self.update_type = Some(update_type);
        self
    }

    // Restrict results to updates associated with the given user(s).
    pub fn users(mut self, users: Vec<&'a str>) -> Self {
        self.users = Some(users);
        self
    }
}

#[derive(Debug, Serialize)]
struct UpdatePageQuery {
    active_releases: Option<bool>,
    #[serde(rename = "alias")]
    aliases: Option<Vec<String>>,
    #[serde(with = "crate::option_bodhi_date_format")]
    approved_before: Option<BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format")]
    approved_since: Option<BodhiDate>,
    bugs: Option<Vec<u32>>,
    builds: Option<Vec<String>>,
    content_type: Option<ContentType>,
    critpath: Option<bool>,
    cves: Option<Vec<String>>,
    like: Option<String>,
    locked: Option<bool>,
    #[serde(with = "crate::option_bodhi_date_format")]
    modified_before: Option<BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format")]
    modified_since: Option<BodhiDate>,
    packages: Option<Vec<String>>,
    pushed: Option<bool>,
    #[serde(with = "crate::option_bodhi_date_format")]
    pushed_before: Option<BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format")]
    pushed_since: Option<BodhiDate>,
    releases: Option<Vec<FedoraRelease>>,
    request: Option<UpdateRequest>,
    search: Option<String>,
    severity: Option<UpdateSeverity>,
    status: Option<UpdateStatus>,
    #[serde(with = "crate::option_bodhi_date_format")]
    submitted_before: Option<BodhiDate>,
    #[serde(with = "crate::option_bodhi_date_format")]
    submitted_since: Option<BodhiDate>,
    suggest: Option<UpdateSuggestion>,
    #[serde(rename = "updateid")]
    update_ids: Option<Vec<String>>,
    #[serde(rename = "type")]
    update_type: Option<UpdateType>,
    #[serde(rename = "user")]
    users: Option<Vec<String>>,

    page: u32,
    rows_per_page: u32,
}

impl SingleRequest<UpdateListPage, Vec<Update>> for UpdatePageQuery {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/updates/?{}", serde_url_params::to_string(self)?))
    }

    fn body(&self) -> Option<String> {
        None
    }

    fn parse(&self, string: &str) -> Result<UpdateListPage, QueryError> {
        let page: UpdateListPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: UpdateListPage) -> Vec<Update> {
        page.updates
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateListPage {
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
    fn page_request(&self, page: u32) -> Box<dyn SingleRequest<UpdateListPage, Vec<Update>>> {
        Box::new(UpdatePageQuery {
            active_releases: self.active_releases,
            aliases: self
                .aliases
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            approved_before: self.approved_before.as_ref().map(|d| (*d).clone()),
            approved_since: self.approved_since.as_ref().map(|d| (*d).clone()),
            bugs: self.bugs.clone(),
            builds: self
                .builds
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            content_type: self.content_type,
            critpath: self.critpath,
            cves: self.cves.as_ref().map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            like: self.like.map(|s| s.to_owned()),
            locked: self.locked,
            modified_before: self.modified_before.as_ref().map(|d| (*d).clone()),
            modified_since: self.modified_since.as_ref().map(|d| (*d).clone()),
            packages: self
                .packages
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            pushed: self.pushed,
            pushed_before: self.pushed_before.as_ref().map(|d| (*d).clone()),
            pushed_since: self.pushed_since.as_ref().map(|d| (*d).clone()),
            releases: self.releases.clone(),
            request: self.request,
            search: self.search.map(|s| s.to_owned()),
            severity: self.severity,
            status: self.status,
            submitted_before: self.submitted_before.as_ref().map(|d| (*d).clone()),
            submitted_since: self.submitted_since.as_ref().map(|d| (*d).clone()),
            suggest: self.suggest,
            update_ids: self
                .update_ids
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            update_type: self.update_type,
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
