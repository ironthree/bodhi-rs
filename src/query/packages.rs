//! The contents of this module can be used to query a bodhi instance about existing packages.
//!
//! The [`PackageQuery`](struct.PackageQuery.html) can be used to execute complex queries, for
//! example query packages by name, or filter packages matching a certain search string.

use std::fmt::{Debug, Formatter};

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
/// # #[cfg(feature = "online-tests")]
/// let packages = bodhi.query(PackageQuery::new().search("rust*")).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/packages.html#service-0>
#[derive(Default)]
pub struct PackageQuery<'a> {
    like: Option<&'a str>,
    name: Option<&'a str>,
    search: Option<&'a str>,

    /// optional callback function for reporting progress
    callback: Option<Box<dyn FnMut(u32, u32) + 'a>>,
}

impl<'a> Debug for PackageQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "PackageQuery {{ like: {:?}, name: {:?}, search: {:?} }}",
            &self.like, &self.name, &self.search,
        )
    }
}

impl<'a> PackageQuery<'a> {
    /// This method returns a new [`PackageQuery`](struct.PackageQuery.html) with *no* filters set.
    pub fn new() -> Self {
        PackageQuery {
            like: None,
            name: None,
            search: None,
            callback: None,
        }
    }

    /// Add a callback function for reporting back query progress for long-running queries.
    /// The function will be called with the current page and the total number of pages for
    /// paginated queries.
    pub fn callback(mut self, fun: impl Fn(u32, u32) + 'a) -> Self {
        self.callback = Some(Box::new(fun));
        self
    }

    /// Restrict search to packages *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    /// Restrict the returned results to packages matching the given name.
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// Restrict search to packages containing the given argument.
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    /// Query the remote bodhi instance with the given parameters.
    async fn query(mut self, bodhi: &BodhiService) -> Result<Vec<Package>, QueryError> {
        let mut packages: Vec<Package> = Vec::new();
        let mut page = 1;

        // initial progress: 0 out of some
        if let Some(ref mut fun) = self.callback {
            fun(0, 1);
        }

        loop {
            let query = self.page_query(page, DEFAULT_ROWS);
            let result = query.query(bodhi).await?;

            if let Some(ref mut fun) = self.callback {
                fun(page, result.pages)
            }

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
            like: self.like,
            name: self.name,
            search: self.search,
            page,
            rows_per_page,
        }
    }
}

#[async_trait::async_trait]
impl<'a> Query<'a, Vec<Package>> for PackageQuery<'a> {
    async fn query(&'a self, bodhi: &'a BodhiService) -> Result<Vec<Package>, QueryError> {
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
    like: Option<&'a str>,
    name: Option<&'a str>,
    search: Option<&'a str>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> SinglePageQuery<PackageListPage> for PackagePageQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/packages/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(string: &str) -> Result<PackageListPage, QueryError> {
        let package_page: PackageListPage = serde_json::from_str(string)?;
        Ok(package_page)
    }

    fn missing() -> Result<PackageListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
