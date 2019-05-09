use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Release};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

#[derive(Debug)]
pub struct ReleaseNameQuery {
    name: String,
}

impl ReleaseNameQuery {
    pub fn new(name: String) -> ReleaseNameQuery {
        ReleaseNameQuery { name }
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Release, String> {
        let path = format!("/releases/{}", self.name);

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let release: Release = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(release)
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

// =================================================================================================

#[derive(Debug, Default)]
pub struct ReleaseQuery {
    exclude_archived: Option<bool>,
    ids: Option<Vec<String>>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    updates: Option<Vec<String>>,
}

impl ReleaseQuery {
    pub fn new() -> ReleaseQuery {
        ReleaseQuery {
            exclude_archived: None,
            ids: None,
            name: None,
            packages: None,
            updates: None,
        }
    }

    pub fn exclude_archived(mut self, exclude_archived: bool) -> ReleaseQuery {
        self.exclude_archived = Some(exclude_archived);
        self
    }

    pub fn id(mut self, id: String) -> ReleaseQuery {
        match &mut self.ids {
            Some(ids) => ids.push(id),
            None => self.ids = Some(vec![id]),
        }

        self
    }

    pub fn name(mut self, name: String) -> ReleaseQuery {
        self.name = Some(name);
        self
    }

    pub fn package(mut self, package: String) -> ReleaseQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    pub fn update(mut self, update: String) -> ReleaseQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Release>, String> {
        let mut overrides: Vec<Release> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = ReleasePageQuery::new().page(page);

            if let Some(exclude_archived) = self.exclude_archived {
                query = query.exclude_archived(exclude_archived);
            };

            if let Some(ids) = self.ids.clone() {
                for id in ids {
                    query = query.id(id);
                }
            };

            if let Some(name) = self.name.clone() {
                query = query.name(name);
            }

            if let Some(packages) = self.packages.clone() {
                for package in packages {
                    query = query.package(package);
                }
            };

            if let Some(updates) = self.updates.clone() {
                for update in updates {
                    query = query.update(update);
                }
            };

            // =====================================================================================

            let result = query.query(bodhi)?;
            overrides.extend(result.releases);

            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(overrides)
    }
}

#[derive(Debug, Deserialize)]
struct ReleaseListPage {
    releases: Vec<Release>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}

#[derive(Debug)]
struct ReleasePageQuery {
    exclude_archived: Option<bool>,
    ids: Option<Vec<String>>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    page: i32,
    rows_per_page: i32,
    updates: Option<Vec<String>>,
}

impl ReleasePageQuery {
    fn new() -> ReleasePageQuery {
        ReleasePageQuery {
            exclude_archived: None,
            ids: None,
            name: None,
            packages: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
            updates: None,
        }
    }

    pub fn exclude_archived(mut self, exclude_archived: bool) -> ReleasePageQuery {
        self.exclude_archived = Some(exclude_archived);
        self
    }

    pub fn id(mut self, id: String) -> ReleasePageQuery {
        match &mut self.ids {
            Some(ids) => ids.push(id),
            None => self.ids = Some(vec![id]),
        }

        self
    }

    pub fn name(mut self, name: String) -> ReleasePageQuery {
        self.name = Some(name);
        self
    }

    pub fn package(mut self, package: String) -> ReleasePageQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    pub fn update(mut self, update: String) -> ReleasePageQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    fn page(mut self, page: i32) -> ReleasePageQuery {
        self.page = page;
        self
    }

    /*
    fn rows_per_page(mut self, rows_per_page: i32) -> ReleasePageQuery {
        self.rows_per_page = rows_per_page;
        self
    }
    */

    fn query(self, bodhi: &BodhiService) -> Result<ReleaseListPage, String> {
        let path = String::from("/releases/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(exclude_archived) = self.exclude_archived {
            args.insert("exclude_archived", exclude_archived.to_string());
        };

        if let Some(ids) = self.ids {
            args.insert("ids", ids.join(","));
        };

        if let Some(name) = self.name {
            args.insert("name", name);
        };

        if let Some(packages) = self.packages {
            args.insert("packages", packages.join(","));
        };

        if let Some(updates) = self.updates {
            args.insert("updates", updates.join(","));
        };

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        let mut response = bodhi.request(&path, Some(args))?;
        let status = response.status();

        if status.is_success() {
            let releases: ReleaseListPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(releases)
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
