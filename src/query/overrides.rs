use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Override};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

#[derive(Debug)]
pub struct OverrideNVRQuery {
    nvr: String,
}

#[derive(Debug, Deserialize)]
struct OverridePage {
    pub r#override: Override,
}

impl OverrideNVRQuery {
    pub fn new(nvr: String) -> OverrideNVRQuery {
        OverrideNVRQuery { nvr }
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Override, String> {
        let path = format!("/overrides/{}", self.nvr);

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let override_page: OverridePage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(override_page.r#override)
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
pub struct OverrideQuery {
    builds: Option<Vec<String>>,
    expired: Option<bool>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<String>>,
    search: Option<String>,
    user: Option<Vec<String>>,
}

impl OverrideQuery {
    pub fn new() -> OverrideQuery {
        OverrideQuery {
            builds: None,
            expired: None,
            like: None,
            packages: None,
            releases: None,
            search: None,
            user: None,
        }
    }

    pub fn build(mut self, build: String) -> OverrideQuery {
        match &mut self.builds {
            Some(builds) => builds.push(build),
            None => self.builds = Some(vec![build]),
        }

        self
    }

    pub fn expired(mut self, expired: bool) -> OverrideQuery {
        self.expired = Some(expired);
        self
    }

    pub fn like(mut self, like: String) -> OverrideQuery {
        self.like = Some(like);
        self
    }

    pub fn package(mut self, package: String) -> OverrideQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    pub fn release(mut self, release: String) -> OverrideQuery {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec![release]),
        }

        self
    }

    pub fn search(mut self, search: String) -> OverrideQuery {
        self.search = Some(search);
        self
    }

    pub fn user(mut self, user: String) -> OverrideQuery {
        match &mut self.user {
            Some(users) => users.push(user),
            None => self.user = Some(vec![user]),
        }

        self
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Override>, String> {
        let mut overrides: Vec<Override> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = OverridePageQuery::new().page(page);

            if let Some(builds) = self.builds.clone() {
                for build in builds {
                    query = query.build(build);
                }
            };

            if let Some(expired) = self.expired {
                query = query.expired(expired);
            };

            if let Some(like) = self.like.clone() {
                query = query.like(like);
            };

            if let Some(packages) = self.packages.clone() {
                for package in packages {
                    query = query.package(package);
                }
            };

            if let Some(releases) = self.releases.clone() {
                for release in releases {
                    query = query.release(release);
                }
            };

            if let Some(search) = self.search.clone() {
                query = query.search(search);
            };

            if let Some(users) = self.user.clone() {
                for user in users {
                    query = query.user(user);
                }
            };

            let result = query.query(bodhi)?;
            overrides.extend(result.r#overrides);

            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(overrides)
    }
}

#[derive(Debug, Deserialize)]
struct OverrideListPage {
    overrides: Vec<Override>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}

#[derive(Debug)]
struct OverridePageQuery {
    builds: Option<Vec<String>>,
    expired: Option<bool>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    page: i32,
    releases: Option<Vec<String>>,
    rows_per_page: i32,
    search: Option<String>,
    user: Option<Vec<String>>,
}

impl OverridePageQuery {
    fn new() -> OverridePageQuery {
        OverridePageQuery {
            builds: None,
            expired: None,
            like: None,
            packages: None,
            page: DEFAULT_PAGE,
            releases: None,
            rows_per_page: DEFAULT_ROWS,
            search: None,
            user: None,
        }
    }

    fn build(mut self, build: String) -> OverridePageQuery {
        match &mut self.builds {
            Some(builds) => builds.push(build),
            None => self.builds = Some(vec![build]),
        }

        self
    }

    fn expired(mut self, expired: bool) -> OverridePageQuery {
        self.expired = Some(expired);
        self
    }

    fn like(mut self, like: String) -> OverridePageQuery {
        self.like = Some(like);
        self
    }

    fn package(mut self, package: String) -> OverridePageQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    fn page(mut self, page: i32) -> OverridePageQuery {
        self.page = page;
        self
    }

    fn release(mut self, release: String) -> OverridePageQuery {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec![release]),
        }

        self
    }

    /*
    fn rows_per_page(mut self, rows_per_page: i32) -> OverridePageQuery {
        self.rows_per_page = rows_per_page;
        self
    }
    */

    fn search(mut self, search: String) -> OverridePageQuery {
        self.search = Some(search);
        self
    }

    fn user(mut self, user: String) -> OverridePageQuery {
        match &mut self.user {
            Some(users) => users.push(user),
            None => self.user = Some(vec![user]),
        }

        self
    }

    fn query(self, bodhi: &BodhiService) -> Result<OverrideListPage, String> {
        let path = String::from("/overrides/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(builds) = self.builds {
            args.insert("builds", builds.join(","));
        }

        if let Some(expired) = self.expired {
            args.insert("expired", expired.to_string());
        }

        if let Some(like) = self.like {
            args.insert("like", like);
        }

        if let Some(packages) = self.packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(releases) = self.releases {
            args.insert("releases", releases.join(","));
        }

        if let Some(search) = self.search {
            args.insert("search", search);
        }

        if let Some(users) = self.user {
            args.insert("user", users.join(","));
        }

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        let mut response = bodhi.request(&path, Some(args))?;
        let status = response.status();

        if status.is_success() {
            let overrides: OverrideListPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(overrides)
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
