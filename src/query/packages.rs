//! The contents of this module can be used to query a bodhi instance about existing packages.
//!
//! The [`PackageQuery`](struct.PackageQuery.html) can be used to execute complex queries, for
//! example query packages by name, or filter packages matching a certain search string.

use serde::{Deserialize, Serialize};

use crate::error::{QueryError, ServiceError};
use crate::service::DEFAULT_ROWS;
use crate::{BodhiService, Package, Query, SinglePageQuery};

/// Use this for querying bodhi about a set of packages with the given properties, which can be
/// specified with the builder pattern. Note that some options can be specified multiple times, and
/// packages will be returned if any criteria match. This is consistent with both the web interface
/// and REST API behavior.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, PackageQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let packages = bodhi.query(&PackageQuery::new().search(String::from("rust*"))).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/packages.html#service-0>
#[derive(Debug, Default)]
pub struct PackageQuery {
    like: Option<String>,
    name: Option<String>,
    search: Option<String>,
}

impl PackageQuery {
    /// This method returns a new [`PackageQuery`](struct.PackageQuery.html) with *no* filters set.
    pub fn new() -> Self {
        PackageQuery {
            like: None,
            name: None,
            search: None,
        }
    }

    /// Restrict search to packages *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: String) -> Self {
        self.like = Some(like);
        self
    }

    /// Restrict the returned results to packages matching the given name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Restrict search to packages containing the given argument.
    pub fn search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }

    /// Query the remote bodhi instance with the given parameters.
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Package>, QueryError> {
        let mut packages: Vec<Package> = Vec::new();
        let mut page = 1;

        loop {
            let query = self.page_query(page, DEFAULT_ROWS);
            let result = query.query(bodhi)?;

            packages.extend(result.packages);
            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(packages)
    }

    fn page_query(&self, page: u32, rows_per_page: u32) -> PackagePageQuery {
        PackagePageQuery {
            like: self.like.as_ref(),
            name: self.name.as_ref(),
            search: self.search.as_ref(),
            page,
            rows_per_page,
        }
    }
}

impl Query<Vec<Package>> for PackageQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Package>, QueryError> {
        PackageQuery::query(self, bodhi)
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

#[derive(Debug, Serialize)]
struct PackagePageQuery<'a> {
    like: Option<&'a String>,
    name: Option<&'a String>,
    search: Option<&'a String>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> SinglePageQuery<PackageListPage> for PackagePageQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/packages/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(string: String) -> Result<PackageListPage, QueryError> {
        let package_page: PackageListPage = serde_json::from_str(&string)?;
        Ok(package_page)
    }

    fn missing() -> Result<PackageListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
