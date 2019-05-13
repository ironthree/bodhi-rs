use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Override};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi for a specific override,
/// by its Name-Version-Release string.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from("https://bodhi.fedoraproject.org"));
///
/// let over_ride = bodhi::OverrideNVRQuery::new(String::from("wingpanel-2.2.1-1.fc28"))
///     .query(&bodhi).unwrap();
/// ```
#[derive(Debug)]
pub struct OverrideNVRQuery {
    nvr: String,
}

#[derive(Debug, Deserialize)]
struct OverridePage {
    r#override: Override,
}

impl OverrideNVRQuery {
    /// This method is the only way to create a new `OverrideNVRQuery` instance.
    pub fn new(nvr: String) -> OverrideNVRQuery {
        OverrideNVRQuery { nvr }
    }

    /// This method will query the remote bodhi instance for the given NVR,
    /// and will return either an `Ok(Override)` matching the specified NVR,
    /// or return an `Err(String)` if it doesn't exist, or if another error occurred.
    ///
    /// TODO: return `Result<Option<Override>, String>` to distinguish "not found" from errors
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

/// Use this for querying bodhi about a set of overrides with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and overrides will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from("https://bodhi.fedoraproject.org"));
///
/// let overrides = bodhi::OverrideQuery::new()
///     .releases(String::from("F29"))
///     .users(String::from("decathorpe"))
///     .query(&bodhi).unwrap();
/// ```
#[derive(Debug, Default)]
pub struct OverrideQuery {
    builds: Option<Vec<String>>,
    expired: Option<bool>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<String>>,
    search: Option<String>,
    users: Option<Vec<String>>,
}

impl OverrideQuery {
    /// This method returns a new `OverrideQuery` with *no* filters set.
    pub fn new() -> OverrideQuery {
        OverrideQuery {
            builds: None,
            expired: None,
            like: None,
            packages: None,
            releases: None,
            search: None,
            users: None,
        }
    }

    /// Restrict the returned results to overrides for the given build(s).
    /// Can be specified multiple times.
    pub fn builds(mut self, build: String) -> OverrideQuery {
        match &mut self.builds {
            Some(builds) => builds.push(build),
            None => self.builds = Some(vec![build]),
        }

        self
    }

    /// Restrict the returned results to (not) expired overrides.
    pub fn expired(mut self, expired: bool) -> OverrideQuery {
        self.expired = Some(expired);
        self
    }

    /// Restrict search to overrides *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: String) -> OverrideQuery {
        self.like = Some(like);
        self
    }

    /// Restrict the returned results to overrides for the given package(s).
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> OverrideQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict the returned results to overrides for the given release(s).
    /// Can be specified multiple times.
    pub fn releases(mut self, release: String) -> OverrideQuery {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec![release]),
        }

        self
    }

    /// Restrict search to overrides containing the given argument.
    pub fn search(mut self, search: String) -> OverrideQuery {
        self.search = Some(search);
        self
    }

    /// Restrict the returned results to overrides created by the given user(s).
    /// Can be specified multiple times.
    pub fn users(mut self, user: String) -> OverrideQuery {
        match &mut self.users {
            Some(users) => users.push(user),
            None => self.users = Some(vec![user]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Override>, String> {
        let mut overrides: Vec<Override> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = OverridePageQuery::new();
            query.page = page;

            query.builds = self.builds.clone();
            query.expired = self.expired.clone();
            query.like = self.like.clone();
            query.packages = self.packages.clone();
            query.releases = self.releases.clone();
            query.search = self.search.clone();
            query.users = self.users.clone();

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
    releases: Option<Vec<String>>,
    search: Option<String>,
    users: Option<Vec<String>>,

    page: i32,
    rows_per_page: i32,
}

impl OverridePageQuery {
    fn new() -> OverridePageQuery {
        OverridePageQuery {
            builds: None,
            expired: None,
            like: None,
            packages: None,
            releases: None,
            search: None,
            users: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
        }
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

        if let Some(users) = self.users {
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
