// ! The contents of this module can be used to query a bodhi instance about existing builds.
// !
// ! The [`BuildNVRQuery`](struct.BuildNVRQuery.html) returns exactly one Build, if and only if a
// ! [`Build`](../../data/struct.Build.html) with the given Name-Version-Release triple exists -
// ! otherwise, it will return an error.
// !
// ! The [`BuildQuery`](struct.BuildQuery.html) can be used to execute more complex queries -
// ! querying builds of certain packages, builds for certain releases, or builds associated with a
// ! given set of updates is possible.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::data::{Build, FedoraRelease};
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};
use crate::service::DEFAULT_ROWS;

// Use this for querying bodhi for a specific build, by its NVR (Name-Version-Release) string. It
// will either return an `Ok(Some(Build))` matching the specified NVR, return `Ok(None)` if it
// doesn't exist, or return an `Err(QueryError)` if another error occurred.
//
// ```
// # use bodhi::{BuildNVRQuery, BodhiServiceBuilder};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let build = bodhi.query(BuildNVRQuery::new("rust-1.34.1-1.fc29")).unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/builds.html#service-0>
#[derive(Debug)]
pub struct BuildNVRQuery<'a> {
    // NVR of the build to query (Name-Version-Release format, without Epoch)
    nvr: &'a str,
}

impl<'a> BuildNVRQuery<'a> {
    // This method is the only way to create a new [`BuildNVRQuery`](struct.BuildNVRQuery.html)
    // instance.
    pub fn new(nvr: &'a str) -> Self {
        BuildNVRQuery { nvr }
    }
}

impl<'a> SingleRequest<Build, Build> for BuildNVRQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/builds/{}", self.nvr))
    }

    fn parse(&self, string: &str) -> Result<Build, QueryError> {
        let build: Build = serde_json::from_str(string)?;
        Ok(build)
    }

    fn extract(&self, page: Build) -> Build {
        page
    }
}

// Use this for querying bodhi about a set of builds with the given properties, which can be
// specified with the builder pattern. Note that some options can be specified multiple times, and
// builds will be returned if any criteria match. This is consistent with both the web interface
// and REST API behavior.
//
// ```
// # use bodhi::{BuildQuery, FedoraRelease, BodhiServiceBuilder};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let builds = bodhi
//     .query(
//         BuildQuery::new()
//             .releases(vec![FedoraRelease::F30, FedoraRelease::F29])
//             .packages(vec!["rust"]),
//     )
//     .unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/builds.html#service-1>
#[derive(Default)]
pub struct BuildQuery<'a> {
    // NVR of the build to query (Name-Version-Release format, without Epoch)
    nvr: Option<&'a str>,
    // list of packages to request builds for
    packages: Option<Vec<&'a str>>,
    // list of releases to request builds for
    releases: Option<Vec<&'a FedoraRelease>>,
    // list of updates to request builds for
    updates: Option<Vec<&'a str>>,

    // number of results per page
    rows_per_page: u32,
    // optional callback function for reporting progress
    callback: Option<Box<dyn Fn(u32, u32) + 'a>>,
}

impl<'a> Debug for BuildQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("BuildQuery")
            .field("nvr", &self.nvr)
            .field("packages", &self.packages)
            .field("releases", &self.releases)
            .field("updates", &self.updates)
            .field("rows_per_page", &self.rows_per_page)
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> BuildQuery<'a> {
    // This method returns a new [`BuildQuery`](struct.BuildQuery.html) with *no* filters set.
    pub fn new() -> Self {
        BuildQuery {
            rows_per_page: DEFAULT_ROWS,
            ..Default::default()
        }
    }

    // Override the maximum number of results per page (capped at 100 server-side).
    #[must_use]
    pub fn rows_per_page(mut self, rows_per_page: u32) -> Self {
        self.rows_per_page = rows_per_page;
        self
    }

    // Add a callback function for reporting back query progress for long-running queries.
    // The function will be called with the current page and the total number of pages for
    // paginated queries.
    #[must_use]
    pub fn callback(mut self, fun: impl Fn(u32, u32) + 'a) -> Self {
        self.callback = Some(Box::new(fun));
        self
    }

    // Restrict the returned results to builds with the given NVR. If this is the only required
    // filter, consider using a [`BuildNVRQuery`](struct.BuildNVRQuery.html) instead.
    #[must_use]
    pub fn nvr(mut self, nvr: &'a str) -> Self {
        self.nvr = Some(nvr);
        self
    }

    // Restrict the returned results to builds of the given package(s).
    #[must_use]
    pub fn packages(mut self, packages: Vec<&'a str>) -> Self {
        self.packages = Some(packages);
        self
    }

    // Restrict the returned results to builds for the given release(s).
    #[must_use]
    pub fn releases(mut self, releases: Vec<&'a FedoraRelease>) -> Self {
        self.releases = Some(releases);
        self
    }

    // Restrict the returned results to builds for the given update(s).
    #[must_use]
    pub fn updates(mut self, updates: Vec<&'a str>) -> Self {
        self.updates = Some(updates);
        self
    }
}

#[derive(Debug, Serialize)]
struct BuildPageQuery {
    nvr: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<FedoraRelease>>,
    updates: Option<Vec<String>>,

    page: u32,
    rows_per_page: u32,
}

impl SingleRequest<BuildListPage, Vec<Build>> for BuildPageQuery {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/builds/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(&self, string: &str) -> Result<BuildListPage, QueryError> {
        let page: BuildListPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: BuildListPage) -> Vec<Build> {
        page.builds
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BuildListPage {
    builds: Vec<Build>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

impl Pagination for BuildListPage {
    fn pages(&self) -> u32 {
        self.pages
    }
}

impl<'a> PaginatedRequest<BuildListPage, Vec<Build>> for BuildQuery<'a> {
    fn page_request(&self, page: u32) -> Box<dyn SingleRequest<BuildListPage, Vec<Build>>> {
        Box::new(BuildPageQuery {
            nvr: self.nvr.map(|s| s.to_owned()),
            packages: self
                .packages
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            releases: self
                .releases
                .as_ref()
                .map(|v| v.iter().map(|r| (*r).to_owned()).collect()),
            updates: self
                .updates
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            page,
            rows_per_page: self.rows_per_page,
        })
    }

    fn callback(&self, page: u32, pages: u32) {
        if let Some(ref callback) = &self.callback {
            callback(page, pages)
        }
    }
}
