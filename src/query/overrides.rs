// ! The contents of this module can be used to query a bodhi instance about existing buildroot
// ! overrides.
// !
// ! The [`OverrideNVRQuery`](struct.OverrideNVRQuery.html) returns exactly one
// ! [`Override`](../../data/struct.Override.html), if and only if an `Override` for the build with
// ! this NVR exists - otherwise, it will return an error.
// !
// ! The [`OverrideQuery`](struct.OverrideQuery.html) can be used to execute more complex queries,
// ! for example filtering overrides by status, sets of overrides for certain packages, or overrides
// ! filed by a given list of users.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::data::{FedoraRelease, Override};
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};
use crate::service::DEFAULT_ROWS;

// Use this for querying bodhi for a specific override, by its NVR (Name-Version-Release) string.
// It will return either an `Ok(Some(Override))` matching the specified NVR, return `Ok(None)` if
// it doesn't exist, or return an `Err(QueryError)` if another error occurred.
//
// ```
// # use bodhi::{BodhiServiceBuilder, OverrideNVRQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let over_ride = bodhi.query(OverrideNVRQuery::new("wingpanel-2.2.1-1.fc28")).unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/overrides.html#service-0>
#[derive(Debug)]
pub struct OverrideNVRQuery<'a> {
    nvr: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct OverridePage {
    r#override: Override,
}

impl<'a> OverrideNVRQuery<'a> {
    // This method is the only way to create a new
    // [`OverrideNVRQuery`](struct.OverrideNVRQuery.html) instance.
    pub fn new(nvr: &'a str) -> Self {
        OverrideNVRQuery { nvr }
    }
}

impl<'a> SingleRequest<OverridePage, Override> for OverrideNVRQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/overrides/{}", self.nvr))
    }

    fn parse(&self, string: &str) -> Result<OverridePage, QueryError> {
        let page: OverridePage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: OverridePage) -> Override {
        page.r#override
    }
}

// Use this for querying bodhi about a set of overrides with the given properties, which can be
// specified with the builder pattern. Note that some options can be specified multiple times, and
// overrides will be returned if any criteria match. This is consistent with both the web interface
// and REST API behavior.
//
// ```
// # use bodhi::{BodhiServiceBuilder, FedoraRelease, OverrideQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let overrides = bodhi
//     .query(
//         OverrideQuery::new()
//             .releases(vec![FedoraRelease::F29])
//             .users(vec!["decathorpe"]),
//     )
//     .unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/overrides.html#service-1>
#[derive(Default)]
pub struct OverrideQuery<'a> {
    builds: Option<Vec<&'a str>>,
    expired: Option<bool>,
    like: Option<&'a str>,
    packages: Option<Vec<&'a str>>,
    releases: Option<Vec<FedoraRelease>>,
    search: Option<&'a str>,
    users: Option<Vec<&'a str>>,

    // optional callback function for reporting progress
    callback: Option<Box<dyn Fn(u32, u32) + 'a>>,
}

impl<'a> Debug for OverrideQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("OverrideQuery")
            .field("builds", &self.builds)
            .field("expired", &self.expired)
            .field("like", &self.like)
            .field("packages", &self.packages)
            .field("releases", &self.releases)
            .field("search", &self.search)
            .field("users", &self.users)
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> OverrideQuery<'a> {
    // This method returns a new [`OverrideQuery`](struct.OverrideQuery.html) with *no* filters
    // set.
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

    // Restrict the returned results to overrides for the given build(s).
    pub fn builds(mut self, builds: Vec<&'a str>) -> Self {
        self.builds = Some(builds);
        self
    }

    // Restrict the returned results to (not) expired overrides.
    pub fn expired(mut self, expired: bool) -> Self {
        self.expired = Some(expired);
        self
    }

    // Restrict search to overrides *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    // Restrict the returned results to overrides for the given package(s).
    pub fn packages(mut self, packages: Vec<&'a str>) -> Self {
        self.packages = Some(packages);
        self
    }

    // Restrict the returned results to overrides for the given release(s).
    pub fn releases(mut self, releases: Vec<FedoraRelease>) -> Self {
        self.releases = Some(releases);
        self
    }

    // Restrict search to overrides containing the given argument.
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    // Restrict the returned results to overrides created by the given user(s).
    pub fn users(mut self, users: Vec<&'a str>) -> Self {
        self.users = Some(users);
        self
    }
}

#[derive(Debug, Serialize)]
struct OverridePageQuery {
    builds: Option<Vec<String>>,
    expired: Option<bool>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<FedoraRelease>>,
    search: Option<String>,
    #[serde(rename = "user")]
    users: Option<Vec<String>>,

    page: u32,
    rows_per_page: u32,
}

impl SingleRequest<OverrideListPage, Vec<Override>> for OverridePageQuery {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/overrides/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(&self, string: &str) -> Result<OverrideListPage, QueryError> {
        let page: OverrideListPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: OverrideListPage) -> Vec<Override> {
        page.overrides
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct OverrideListPage {
    overrides: Vec<Override>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

impl Pagination for OverrideListPage {
    fn pages(&self) -> u32 {
        self.pages
    }
}

impl<'a> PaginatedRequest<OverrideListPage, Vec<Override>> for OverrideQuery<'a> {
    fn page_request(&self, page: u32) -> Box<dyn SingleRequest<OverrideListPage, Vec<Override>>> {
        Box::new(OverridePageQuery {
            builds: self
                .builds
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            expired: self.expired,
            like: self.like.map(|s| s.to_owned()),
            packages: self
                .packages
                .as_ref()
                .map(|v| v.iter().map(|s| (*s).to_owned()).collect()),
            releases: self.releases.as_ref().map(|v| v.iter().map(|r| r.to_owned()).collect()),
            search: self.search.map(|s| s.to_owned()),
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
