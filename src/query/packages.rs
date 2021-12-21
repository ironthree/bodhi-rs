// ! The contents of this module can be used to query a bodhi instance about existing packages.
// !
// ! The [`PackageQuery`](struct.PackageQuery.html) can be used to execute complex queries, for
// ! example query packages by name, or filter packages matching a certain search string.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::data::Package;
use crate::error::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};
use crate::service::DEFAULT_ROWS;

// Use this for querying bodhi about a set of packages with the given properties, which can be
// specified with the builder pattern. Note that some options can be specified multiple times, and
// packages will be returned if any criteria match. This is consistent with both the web interface
// and REST API behavior.
//
// ```
// # use bodhi::{BodhiServiceBuilder, PackageQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let packages = bodhi.query(PackageQuery::new().search("rust*")).unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/packages.html#service-0>
#[derive(Default)]
pub struct PackageQuery<'a> {
    like: Option<&'a str>,
    name: Option<&'a str>,
    search: Option<&'a str>,

    // optional callback function for reporting progress
    callback: Option<Box<dyn Fn(u32, u32) + 'a>>,
}

impl<'a> Debug for PackageQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("PackageQuery")
            .field("like", &self.like)
            .field("name", &self.name)
            .field("search", &self.search)
            .field("callback", &"(function pointer)")
            .finish()
    }
}

impl<'a> PackageQuery<'a> {
    // This method returns a new [`PackageQuery`](struct.PackageQuery.html) with *no* filters set.
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

    // Restrict search to packages *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    // Restrict the returned results to packages matching the given name.
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    // Restrict search to packages containing the given argument.
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }
}

#[derive(Debug, Serialize)]
struct PackagePageQuery {
    like: Option<String>,
    name: Option<String>,
    search: Option<String>,

    page: u32,
    rows_per_page: u32,
}

impl SingleRequest<PackageListPage, Vec<Package>> for PackagePageQuery {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/packages/?{}", serde_url_params::to_string(self)?))
    }

    fn body(&self) -> Option<String> {
        None
    }

    fn parse(&self, string: &str) -> Result<PackageListPage, QueryError> {
        let page: PackageListPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: PackageListPage) -> Vec<Package> {
        page.packages
    }
}

#[derive(Debug, Deserialize)]
struct PackageListPage {
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
    fn page_request(&self, page: u32) -> Box<dyn SingleRequest<PackageListPage, Vec<Package>>> {
        Box::new(PackagePageQuery {
            like: self.like.map(|s| s.to_owned()),
            name: self.name.map(|s| s.to_owned()),
            search: self.search.map(|s| s.to_owned()),
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
