use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Build};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

#[derive(Debug)]
pub struct BuildNVRQuery {
    nvr: String,
}

impl BuildNVRQuery {
    pub fn new(nvr: String) -> BuildNVRQuery {
        BuildNVRQuery { nvr }
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Build, String> {
        let path = format!("/builds/{}", self.nvr);

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let build: Build = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(build)
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
pub struct BuildQuery {
    nvr: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<String>>,
    updates: Option<Vec<String>>,
}

impl BuildQuery {
    pub fn new() -> BuildQuery {
        BuildQuery {
            nvr: None,
            packages: None,
            releases: None,
            updates: None,
        }
    }

    pub fn nvr(mut self, nvr: String) -> BuildQuery {
        self.nvr = Some(nvr);
        self
    }

    pub fn packages(mut self, package: String) -> BuildQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    pub fn releases(mut self, release: String) -> BuildQuery {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec![release]),
        }

        self
    }

    pub fn updates(mut self, update: String) -> BuildQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Build>, String> {
        let mut builds: Vec<Build> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = BuildPageQuery::new();
            query.page = page;

            query.nvr = self.nvr.clone();
            query.packages = self.packages.clone();
            query.releases = self.releases.clone();
            query.updates = self.updates.clone();

            let result = query.query(bodhi)?;
            builds.extend(result.builds);

            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(builds)
    }
}

#[derive(Debug, Deserialize)]
struct BuildListPage {
    builds: Vec<Build>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}

#[derive(Debug)]
struct BuildPageQuery {
    nvr: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<String>>,
    updates: Option<Vec<String>>,

    page: i32,
    rows_per_page: i32,
}

impl BuildPageQuery {
    fn new() -> BuildPageQuery {
        BuildPageQuery {
            nvr: None,
            packages: None,
            releases: None,
            updates: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
        }
    }

    fn query(self, bodhi: &BodhiService) -> Result<BuildListPage, String> {
        let path = String::from("/builds/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(nvr) = self.nvr {
            args.insert("nvr", nvr);
        }

        if let Some(packages) = self.packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(releases) = self.releases {
            args.insert("releases", releases.join(","));
        }

        if let Some(updates) = self.updates {
            args.insert("updates", updates.join(","));
        }

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        let mut response = bodhi.request(&path, Some(args))?;
        let status = response.status();

        if status.is_success() {
            let builds: BuildListPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(builds)
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
