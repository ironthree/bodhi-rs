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

    pub fn ids(mut self, id: String) -> ReleaseQuery {
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

    pub fn packages(mut self, package: String) -> ReleaseQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    pub fn updates(mut self, update: String) -> ReleaseQuery {
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
            let mut query = ReleasePageQuery::new();
            query.page = page;

            query.exclude_archived = self.exclude_archived;
            query.ids = self.ids.clone();
            query.name = self.name.clone();
            query.packages = self.packages.clone();
            query.updates = self.updates.clone();

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
    updates: Option<Vec<String>>,

    page: i32,
    rows_per_page: i32,
}

impl ReleasePageQuery {
    fn new() -> ReleasePageQuery {
        ReleasePageQuery {
            exclude_archived: None,
            ids: None,
            name: None,
            packages: None,
            updates: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
        }
    }

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
