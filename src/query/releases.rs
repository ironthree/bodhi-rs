// ! The contents of this module can be used to query a bodhi instance about existing releases.
// !
// ! The [`ReleaseNameQuery`](struct.ReleaseNameQuery.html) returns exactly one
// ! [`Release`](../../data/struct.Release.html), if and only if a `Release` with this name exists -
// ! otherwise, it will return an error.
// !
// ! The [`ReleaseQuery`](struct.ReleaseQuery.html) can be used to execute more complex queries, for
// ! example filtering releases by status, or query the releases associated with a given set of
// ! updates or packages.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::data::Release;
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};
use crate::service::DEFAULT_ROWS;

// Use this for querying bodhi for a specific release by its name. It will either return an
// `Ok(Some(Release))` matching the specified name, return `Ok(None)` if it doesn't exist, or
// return an `Err(QueryError)` if another error occurred.
//
// ```
// # use bodhi::{BodhiServiceBuilder, FedoraRelease, ReleaseNameQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let release = bodhi.query(ReleaseNameQuery::new("F30")).unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/releases.html#service-0>
#[derive(Debug)]
pub struct ReleaseNameQuery<'a> {
    name: &'a str,
}

impl<'a> ReleaseNameQuery<'a> {
    // This method is the only way to create a new
    // [`ReleaseNameQuery`](struct.ReleaseNameQuery.html) instance.
    pub fn new(name: &'a str) -> Self {
        ReleaseNameQuery { name }
    }
}

impl<'a> SingleRequest<Release, Release> for ReleaseNameQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/releases/{}", self.name))
    }

    fn body(&self) -> Option<String> {
        None
    }

    fn parse(&self, string: &str) -> Result<Release, QueryError> {
        let page: Release = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: Release) -> Release {
        page
    }
}

// Use this for querying bodhi about a set of releases with the given properties, which can be
// specified with the builder pattern. Note that some options can be specified multiple times, and
// comments will be returned if any criteria match. This is consistent with both the web interface
// and REST API behavior.
//
// ```
// # use bodhi::{BodhiServiceBuilder, ReleaseQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let releases = bodhi.query(ReleaseQuery::new().exclude_archived(true)).unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/releases.html#service-1>
#[derive(Default)]

pub struct ReleaseQuery<'a> {
    exclude_archived: Option<bool>,
    ids: Option<Vec<&'a str>>,
    name: Option<&'a str>,
    packages: Option<Vec<&'a str>>,
    updates: Option<Vec<&'a str>>,

    // optional callback function for reporting progress
    callback: Option<Box<dyn Fn(u32, u32) + 'a>>,
}

impl<'a> Debug for ReleaseQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("ReleaseQuery")
            .field("exclude_archived", &self.exclude_archived)
            .field("ids", &self.ids)
            .field("name", &self.name)
            .field("packages", &self.packages)
            .field("updates", &self.updates)
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> ReleaseQuery<'a> {
    // This method returns a new [`ReleaseQuery`](struct.ReleaseQuery.html) with *no* filters set.
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

    // Restrict the returned results to (not) archived releases.
    pub fn exclude_archived(mut self, exclude_archived: bool) -> Self {
        self.exclude_archived = Some(exclude_archived);
        self
    }

    // Restrict results to releases with the given ID.
    pub fn ids(mut self, ids: Vec<&'a str>) -> Self {
        self.ids = Some(ids);
        self
    }

    // Restrict results to a release with the given name. If this is the only required filter,
    // consider using a [`ReleaseNameQuery`](struct.ReleaseNameQuery.html) instead.
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    // Restrict the returned results to releases containing the given package(s).
    pub fn packages(mut self, packages: Vec<&'a str>) -> Self {
        self.packages = Some(packages);
        self
    }

    // Restrict the returned results to releases matching the given updates(s).
    pub fn updates(mut self, updates: Vec<&'a str>) -> Self {
        self.updates = Some(updates);
        self
    }
}

#[derive(Debug, Serialize)]
struct ReleasePageQuery {
    exclude_archived: Option<bool>,
    ids: Option<Vec<String>>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    updates: Option<Vec<String>>,

    page: u32,
    rows_per_page: u32,
}

impl SingleRequest<ReleaseListPage, Vec<Release>> for ReleasePageQuery {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/releases/?{}", serde_url_params::to_string(self)?))
    }

    fn body(&self) -> Option<String> {
        None
    }

    fn parse(&self, string: &str) -> Result<ReleaseListPage, QueryError> {
        let page: ReleaseListPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: ReleaseListPage) -> Vec<Release> {
        page.releases
    }
}

#[derive(Debug, Deserialize)]
pub struct ReleaseListPage {
    releases: Vec<Release>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

impl Pagination for ReleaseListPage {
    fn pages(&self) -> u32 {
        self.pages
    }
}

impl<'a> PaginatedRequest<ReleaseListPage, Vec<Release>> for ReleaseQuery<'a> {
    fn page_request(&self, page: u32) -> Box<dyn SingleRequest<ReleaseListPage, Vec<Release>>> {
        Box::new(ReleasePageQuery {
            exclude_archived: self.exclude_archived,
            ids: self.ids.as_ref().map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            name: self.name.map(|s| s.to_owned()),
            packages: self
                .packages
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            updates: self
                .updates
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
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
