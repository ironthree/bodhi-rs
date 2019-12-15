//! query for packages
//!
//! The contents of this module can be used to query a bodhi instance about
//! existing packages.
//!
//! The `Package` can be used to execute complex queries, for example query
//! packages by name, or filter packages matching a certain search string.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::Package;
use crate::error::QueryError;
use crate::query::SinglePageQuery;
use crate::service::{BodhiService, ServiceError, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi about a set of packages with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and packages will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::default().build().unwrap();
///
/// let packages = bodhi::query::PackageQuery::new()
///     .search(String::from("rust*"))
///     .query(&bodhi).unwrap();
/// ```
#[derive(Debug, Default)]
pub struct PackageQuery {
    like: Option<String>,
    name: Option<String>,
    search: Option<String>,
}

impl PackageQuery {
    /// This method returns a new `PackageQuery` with *no* filters set.
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
    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Package>, QueryError> {
        let mut packages: Vec<Package> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = PackagePageQuery::new();
            query.page = page;

            query.like = self.like.clone();
            query.name = self.name.clone();
            query.search = self.search.clone();

            let result = query.query(bodhi)?;
            packages.extend(result.packages);

            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(packages)
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

#[derive(Debug)]
struct PackagePageQuery {
    like: Option<String>,
    name: Option<String>,
    search: Option<String>,

    page: u32,
    rows_per_page: u32,
}

impl PackagePageQuery {
    fn new() -> Self {
        PackagePageQuery {
            like: None,
            name: None,
            search: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
        }
    }
}

impl SinglePageQuery for PackagePageQuery {
    type Output = PackageListPage;

    fn path(&self) -> String {
        String::from("/packages/")
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(like) = &self.like {
            args.insert("like", like.to_owned());
        }

        if let Some(name) = &self.name {
            args.insert("name", name.to_owned());
        }

        if let Some(search) = &self.search {
            args.insert("search", search.to_owned());
        }

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        Some(args)
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
