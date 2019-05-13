use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Package};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi about a set of packages with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and packages will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from("https://bodhi.fedoraproject.org"));
///
/// let packages = bodhi::PackageQuery::new()
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
    pub fn new() -> PackageQuery {
        PackageQuery {
            like: None,
            name: None,
            search: None,
        }
    }

    /// Restrict search to packages *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: String) -> PackageQuery {
        self.like = Some(like);
        self
    }

    /// Restrict the returned results to packages matching the given name.
    pub fn name(mut self, name: String) -> PackageQuery {
        self.name = Some(name);
        self
    }

    /// Restrict search to packages containing the given argument.
    pub fn search(mut self, search: String) -> PackageQuery {
        self.search = Some(search);
        self
    }

    /// Query the remote bodhi instance with the given parameters.
    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Package>, String> {
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
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}

#[derive(Debug)]
struct PackagePageQuery {
    like: Option<String>,
    name: Option<String>,
    search: Option<String>,

    page: i32,
    rows_per_page: i32,
}

impl PackagePageQuery {
    fn new() -> PackagePageQuery {
        PackagePageQuery {
            like: None,
            name: None,
            search: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
        }
    }

    fn query(self, bodhi: &BodhiService) -> Result<PackageListPage, String> {
        let path = String::from("/packages/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(like) = self.like {
            args.insert("like", like);
        }

        if let Some(name) = self.name {
            args.insert("name", name);
        }

        if let Some(search) = self.search {
            args.insert("search", search);
        }

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        let mut response = bodhi.request(&path, Some(args))?;
        let status = response.status();

        if status.is_success() {
            let packages: PackageListPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(packages)
        } else {
            let error: BodhiError = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("Unexpected error message: {:?}", error));
                }
            };

            Err(format!("{:?}", error))
        }
    }
}
