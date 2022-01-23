use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::client::DEFAULT_ROWS;
use crate::data::{FedoraRelease, Override};
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

/// data type encapsulating parameters for querying for a [`Override`] by NVR
///
/// If no override with the specified NVR is known to bodhi, a [`QueryError::NotFound`] error is
/// returned for the query.
///
/// ```
/// use bodhi::OverrideNVRQuery;
///
/// let query = OverrideNVRQuery::new("wingpanel-2.2.1-1.fc28");
/// // let override = bodhi.request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/overrides.html#service-0>
#[derive(Debug)]
pub struct OverrideNVRQuery<'a> {
    nvr: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct OverridePage {
    #[serde(rename = "override")]
    over_ride: Override,
}

impl<'a> OverrideNVRQuery<'a> {
    /// constructor for [`OverrideNVRQuery`] from an NVR string
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
        page.over_ride
    }
}


/// data type encapsulating parameters for querying buildroot [`Override`]s
///
/// ```
/// use bodhi::{ContentType, FedoraRelease, OverrideQuery};
///
/// let releases = vec![FedoraRelease::fedora(34, ContentType::RPM).unwrap()];
/// let query = OverrideQuery::new().releases(&releases).users(&["decathorpe"]);
/// // let overrides = bodhi.paginated_request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/overrides.html#service-1>
#[derive(Default)]
pub struct OverrideQuery<'a> {
    builds: Option<&'a [&'a str]>,
    expired: Option<bool>,
    like: Option<&'a str>,
    packages: Option<&'a [&'a str]>,
    releases: Option<&'a [FedoraRelease]>,
    search: Option<&'a str>,
    users: Option<&'a [&'a str]>,

    // number of results per page
    rows_per_page: u32,
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
            .field("rows_per_page", &self.rows_per_page)
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> OverrideQuery<'a> {
    /// constructor for [`OverrideQuery`] without any filters
    pub fn new() -> Self {
        OverrideQuery {
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

    /// restrict query to overrides matching specific build NVRs
    #[must_use]
    pub fn builds(mut self, builds: &'a [&'a str]) -> Self {
        self.builds = Some(builds);
        self
    }

    /// restrict query to overrides that are (not) expired
    #[must_use]
    pub fn expired(mut self, expired: bool) -> Self {
        self.expired = Some(expired);
        self
    }

    /// restrict query to overrides with notes that are "like" a given string (in the SQL sense)
    #[must_use]
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    /// restrict query to overrides matching specific packages
    #[must_use]
    pub fn packages(mut self, packages: &'a [&'a str]) -> Self {
        self.packages = Some(packages);
        self
    }

    /// restrict query to overrides matching specific releases
    #[must_use]
    pub fn releases(mut self, releases: &'a [FedoraRelease]) -> Self {
        self.releases = Some(releases);
        self
    }

    /// restrict query to overrides matching a search keyword
    #[must_use]
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    /// restrict query to overrides submitted by specific users (identified by their username)
    #[must_use]
    pub fn users(mut self, users: &'a [&'a str]) -> Self {
        self.users = Some(users);
        self
    }
}


/// data type encapsulating parameters for querying specific [`OverrideQuery`] result pages
#[derive(Debug, Serialize)]
pub struct OverridePageQuery<'a> {
    builds: Option<&'a [&'a str]>,
    expired: Option<bool>,
    like: Option<&'a str>,
    packages: Option<&'a [&'a str]>,
    releases: Option<&'a [FedoraRelease]>,
    search: Option<&'a str>,
    #[serde(rename = "user")]
    users: Option<&'a [&'a str]>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> OverridePageQuery<'a> {
    /// constructor for [`OverridePageQuery`] taking parameters from an existing [`OverrideQuery`]
    pub fn from_query(query: &'a OverrideQuery, page: u32) -> Self {
        OverridePageQuery {
            builds: query.builds,
            expired: query.expired,
            like: query.like,
            packages: query.packages,
            releases: query.releases,
            search: query.search,
            users: query.users,
            page,
            rows_per_page: query.rows_per_page,
        }
    }
}

impl<'a> SingleRequest<OverrideListPage, Vec<Override>> for OverridePageQuery<'a> {
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
    fn page_request<'b>(&'b self, page: u32) -> Box<dyn SingleRequest<OverrideListPage, Vec<Override>> + 'b> {
        Box::new(OverridePageQuery::from_query(self, page))
    }

    fn callback(&self, page: u32, pages: u32) {
        if let Some(ref callback) = &self.callback {
            callback(page, pages)
        }
    }
}
