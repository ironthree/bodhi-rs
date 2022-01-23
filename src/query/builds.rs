use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::client::DEFAULT_ROWS;
use crate::data::{Build, FedoraRelease};
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

/// data type encapsulating parameters for querying for a [`Build`] by NVR
///
/// If no build with the specified NVR is known to bodhi, a [`QueryError::NotFound`] error is
/// returned for the query.
///
/// ```
/// use bodhi::BuildNVRQuery;
///
/// let query = BuildNVRQuery::new("rust-1.34.1-1.fc29");
/// // let build = bodhi.request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/builds.html#service-0>
#[derive(Debug)]
pub struct BuildNVRQuery<'a> {
    // NVR of the build to query (Name-Version-Release format, without Epoch)
    nvr: &'a str,
}

impl<'a> BuildNVRQuery<'a> {
    /// constructor for [`BuildNVRQuery`] from an NVR string
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


/// data type encapsulating parameters for querying [`Build`]s
///
/// ```
/// use bodhi::{BuildQuery, ContentType, FedoraRelease};
///
/// let query = BuildQuery::new();
/// // let builds = bodhi.paginated_request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/builds.html#service-1>
#[derive(Default)]
pub struct BuildQuery<'a> {
    nvr: Option<&'a str>,
    packages: Option<&'a [&'a str]>,
    releases: Option<&'a [FedoraRelease]>,
    updates: Option<&'a [&'a str]>,

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
    /// constructor for [`BuildQuery`] without any filters
    pub fn new() -> Self {
        BuildQuery {
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

    /// restrict query to builds matching a specific NVR
    ///
    /// If this is the only parameter, consider using a [`BuildNVRQuery`] instead.
    #[must_use]
    pub fn nvr(mut self, nvr: &'a str) -> Self {
        self.nvr = Some(nvr);
        self
    }

    /// restrict query to builds matching specific packages
    #[must_use]
    pub fn packages(mut self, packages: &'a [&'a str]) -> Self {
        self.packages = Some(packages);
        self
    }

    /// restrict query to builds matching specific releases
    #[must_use]
    pub fn releases(mut self, releases: &'a [FedoraRelease]) -> Self {
        self.releases = Some(releases);
        self
    }

    /// restrict query to builds matching specific updates
    #[must_use]
    pub fn updates(mut self, updates: &'a [&'a str]) -> Self {
        self.updates = Some(updates);
        self
    }
}


/// data type encapsulating parameters for querying specific [`BuildQuery`] result pages
#[derive(Debug, Serialize)]
pub struct BuildPageQuery<'a> {
    nvr: Option<&'a str>,
    packages: Option<&'a [&'a str]>,
    releases: Option<&'a [FedoraRelease]>,
    updates: Option<&'a [&'a str]>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> BuildPageQuery<'a> {
    /// constructor for [`BuildPageQuery`] taking parameters from an existing [`BuildQuery`]
    pub fn from_query(query: &'a BuildQuery, page: u32) -> Self {
        BuildPageQuery {
            nvr: query.nvr,
            packages: query.packages,
            releases: query.releases,
            updates: query.updates,
            page,
            rows_per_page: query.rows_per_page,
        }
    }
}

impl<'a> SingleRequest<BuildListPage, Vec<Build>> for BuildPageQuery<'a> {
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
    fn page_request<'b>(&'b self, page: u32) -> Box<dyn SingleRequest<BuildListPage, Vec<Build>> + 'b> {
        Box::new(BuildPageQuery::from_query(self, page))
    }

    fn callback(&self, page: u32, pages: u32) {
        if let Some(ref callback) = &self.callback {
            callback(page, pages)
        }
    }
}
