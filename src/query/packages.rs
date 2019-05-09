use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Package};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

#[derive(Debug, Default)]
pub struct PackageQuery {
    like: Option<String>,
    name: Option<String>,
    search: Option<String>,
}

impl PackageQuery {
    pub fn new() -> PackageQuery {
        PackageQuery {
            like: None,
            name: None,
            search: None,
        }
    }

    pub fn like(mut self, like: String) -> PackageQuery {
        self.like = Some(like);
        self
    }

    pub fn name(mut self, name: String) -> PackageQuery {
        self.name = Some(name);
        self
    }

    pub fn search(mut self, search: String) -> PackageQuery {
        self.search = Some(search);
        self
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Package>, String> {
        let mut packages: Vec<Package> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = PackagePageQuery::new().page(page);

            if let Some(like) = self.like.clone() {
                query = query.like(like);
            }

            if let Some(name) = self.name.clone() {
                query = query.name(name);
            }

            if let Some(search) = self.search.clone() {
                query = query.search(search);
            }

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
pub(crate) struct PackageListPage {
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
    pub page: i32,
    pub rows_per_page: i32,
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

    fn like(mut self, like: String) -> PackagePageQuery {
        self.like = Some(like);
        self
    }

    fn name(mut self, name: String) -> PackagePageQuery {
        self.name = Some(name);
        self
    }

    fn search(mut self, search: String) -> PackagePageQuery {
        self.search = Some(search);
        self
    }

    fn page(mut self, page: i32) -> PackagePageQuery {
        self.page = page;
        self
    }

    /*
    fn rows_per_page(mut self, rows_per_page: i32) -> PackagePageQuery {
        self.rows_per_page = rows_per_page;
        self
    }
    */

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
