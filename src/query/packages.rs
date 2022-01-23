use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::client::DEFAULT_ROWS;
use crate::data::Package;
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

/// data type encapsulating parameters for querying [`Package`]s
///
/// ```
/// use bodhi::PackageQuery;
///
/// let query = PackageQuery::new().search("rust*");
/// // let packages = bodhi.paginated_request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/packages.html#service-0>
#[derive(Default)]
pub struct PackageQuery<'a> {
    like: Option<&'a str>,
    name: Option<&'a str>,
    search: Option<&'a str>,

    // number of results per page
    rows_per_page: u32,
    // optional callback function for reporting progress
    callback: Option<Box<dyn Fn(u32, u32) + 'a>>,
}

impl<'a> Debug for PackageQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("PackageQuery")
            .field("like", &self.like)
            .field("name", &self.name)
            .field("search", &self.search)
            .field("rows_per_page", &self.rows_per_page)
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> PackageQuery<'a> {
    /// constructor for [`PackageQuery`] without any filters
    pub fn new() -> Self {
        PackageQuery {
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

    /// restrict query to packages "like" the given string (in the SQL sense)
    #[must_use]
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    /// restrict query to packages matching a specific name
    #[must_use]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// restrict query to packages matching a search keyword
    #[must_use]
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }
}


/// data type encapsulating parameters for querying specific [`PackageQuery`] result pages
#[derive(Debug, Serialize)]
pub struct PackagePageQuery<'a> {
    like: Option<&'a str>,
    name: Option<&'a str>,
    search: Option<&'a str>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> PackagePageQuery<'a> {
    /// constructor for [`PackagePageQuery`] taking parameters from an existing [`PackageQuery`]
    pub fn from_query(query: &'a PackageQuery, page: u32) -> Self {
        PackagePageQuery {
            like: query.like,
            name: query.name,
            search: query.search,
            page,
            rows_per_page: query.rows_per_page,
        }
    }
}

impl<'a> SingleRequest<PackageListPage, Vec<Package>> for PackagePageQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/packages/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(&self, string: &str) -> Result<PackageListPage, QueryError> {
        let page: PackageListPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: PackageListPage) -> Vec<Package> {
        page.packages
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PackageListPage {
    packages: Vec<Package>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

impl Pagination for PackageListPage {
    fn pages(&self) -> u32 {
        self.pages
    }
}

impl<'a> PaginatedRequest<PackageListPage, Vec<Package>> for PackageQuery<'a> {
    fn page_request<'b>(&'b self, page: u32) -> Box<dyn SingleRequest<PackageListPage, Vec<Package>> + 'b> {
        Box::new(PackagePageQuery::from_query(self, page))
    }

    fn callback(&self, page: u32, pages: u32) {
        if let Some(ref callback) = &self.callback {
            callback(page, pages)
        }
    }
}
