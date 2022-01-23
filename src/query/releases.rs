use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::client::DEFAULT_ROWS;
use crate::data::{FedoraRelease, Release};
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

/// data type encapsulating parameters for querying for a [`Release`] by name
///
/// If no release with the specified name is known to bodhi, a [`QueryError::NotFound`] error is
/// returned for the query.
///
/// ```
/// use bodhi::ReleaseNameQuery;
///
/// let query = ReleaseNameQuery::new("F34");
/// // let release = bodhi.request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/releases.html#service-0>
#[derive(Debug)]
pub struct ReleaseNameQuery<'a> {
    name: Cow<'a, str>,
}

impl<'a> ReleaseNameQuery<'a> {
    /// constructor for [`ReleaseNameQuery`] from a release name
    pub fn new(name: &'a str) -> Self {
        ReleaseNameQuery {
            name: Cow::Borrowed(name),
        }
    }

    /// constructor for [`ReleaseNameQuery`] from a [`FedoraRelease`] value
    pub fn from_release(release: &FedoraRelease) -> Self {
        ReleaseNameQuery {
            name: Cow::Owned(release.to_string()),
        }
    }
}

impl<'a> SingleRequest<Release, Release> for ReleaseNameQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/releases/{}", self.name))
    }

    fn parse(&self, string: &str) -> Result<Release, QueryError> {
        let page: Release = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: Release) -> Release {
        page
    }
}


/// data type encapsulating parameters for querying [`Release`]s
///
/// ```
/// use bodhi::ReleaseQuery;
///
/// let query = ReleaseQuery::new().exclude_archived(true);
/// // let releases = bodhi.paginated_request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/releases.html#service-1>
#[derive(Default)]
pub struct ReleaseQuery<'a> {
    exclude_archived: Option<bool>,
    ids: Option<&'a [&'a str]>,
    name: Option<&'a str>,
    packages: Option<&'a [&'a str]>,
    updates: Option<&'a [&'a str]>,

    // number of results per page
    rows_per_page: u32,
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
            .field("rows_per_page", &self.rows_per_page)
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> ReleaseQuery<'a> {
    /// constructor for [`ReleaseQuery`] without any filters
    pub fn new() -> Self {
        ReleaseQuery {
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

    /// restrict query to releases that have (not) been archived
    #[must_use]
    pub fn exclude_archived(mut self, exclude_archived: bool) -> Self {
        self.exclude_archived = Some(exclude_archived);
        self
    }

    /// restrict query to releases matching the given IDs
    #[must_use]
    pub fn ids(mut self, ids: &'a [&'a str]) -> Self {
        self.ids = Some(ids);
        self
    }

    /// restrict query to releases matching a specific name
    ///
    /// If this is the only parameter, consider using a [`ReleaseNameQuery`] instead.
    #[must_use]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// restrict query to releases which contain the given packages
    #[must_use]
    pub fn packages(mut self, packages: &'a [&'a str]) -> Self {
        self.packages = Some(packages);
        self
    }

    /// restrict query to releases which match the given updates
    #[must_use]
    pub fn updates(mut self, updates: &'a [&'a str]) -> Self {
        self.updates = Some(updates);
        self
    }
}


/// data type encapsulating parameters for querying specific [`ReleaseQuery`] result pages
#[derive(Debug, Serialize)]
pub struct ReleasePageQuery<'a> {
    exclude_archived: Option<bool>,
    ids: Option<&'a [&'a str]>,
    name: Option<&'a str>,
    packages: Option<&'a [&'a str]>,
    updates: Option<&'a [&'a str]>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> ReleasePageQuery<'a> {
    /// constructor for [`ReleasePageQuery`] taking parameters from an existing [`ReleaseQuery`]
    pub fn from_query(query: &'a ReleaseQuery, page: u32) -> Self {
        ReleasePageQuery {
            exclude_archived: query.exclude_archived,
            ids: query.ids,
            name: query.name,
            packages: query.packages,
            updates: query.updates,
            page,
            rows_per_page: query.rows_per_page,
        }
    }
}

impl<'a> SingleRequest<ReleaseListPage, Vec<Release>> for ReleasePageQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/releases/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(&self, string: &str) -> Result<ReleaseListPage, QueryError> {
        let page: ReleaseListPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: ReleaseListPage) -> Vec<Release> {
        page.releases
    }
}

#[allow(dead_code)]
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
    fn page_request<'b>(&'b self, page: u32) -> Box<dyn SingleRequest<ReleaseListPage, Vec<Release>> + 'b> {
        Box::new(ReleasePageQuery::from_query(self, page))
    }

    fn callback(&self, page: u32, pages: u32) {
        if let Some(ref callback) = &self.callback {
            callback(page, pages)
        }
    }
}
