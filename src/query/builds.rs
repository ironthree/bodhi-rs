use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Build};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi for a specific build,
/// by its Name-Version-Release string.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from(bodhi::FEDORA_BODHI_URL));
///
/// let build = bodhi::BuildNVRQuery::new(String::from("rust-1.34.1-1.fc29"))
///     .query(&bodhi).unwrap();
/// ```
#[derive(Debug)]
pub struct BuildNVRQuery {
    nvr: String,
}

impl BuildNVRQuery {
    /// This method is the only way to create a new `BuildNVRQuery` instance.
    pub fn new(nvr: String) -> BuildNVRQuery {
        BuildNVRQuery { nvr }
    }

    /// This method will query the remote bodhi instance for the given NVR,
    /// and will either return an `Ok(Build)` matching the specified NVR,
    /// or return an `Err(String)` if it doesn't exist, or if another error occurred.
    ///
    /// TODO: return `Result<Option<Build>, String>>` to distinguish "not found" from errors
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

/// Use this for querying bodhi about a set of builds with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and builds will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from(bodhi::FEDORA_BODHI_URL));
///
/// let builds = bodhi::BuildQuery::new()
///     .releases(String::from("F30"))
///     .releases(String::from("F29"))
///     .packages(String::from("rust"))
///     .query(&bodhi).unwrap();
/// ```
#[derive(Debug, Default)]
pub struct BuildQuery {
    nvr: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<String>>,
    updates: Option<Vec<String>>,
}

impl BuildQuery {
    /// This method returns a new `BuildQuery` with *no* filters set.
    pub fn new() -> BuildQuery {
        BuildQuery {
            nvr: None,
            packages: None,
            releases: None,
            updates: None,
        }
    }

    /// Restrict the returned results to builds with the given NVR.
    /// If this is the only required filter, consider using a `BuildNVRQuery` instead.
    pub fn nvr(mut self, nvr: String) -> BuildQuery {
        self.nvr = Some(nvr);
        self
    }

    /// Restrict the returned results to builds of the given package(s).
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> BuildQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict the returned results to builds for the given release(s).
    /// Can be specified multiple times.
    pub fn releases(mut self, release: String) -> BuildQuery {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec![release]),
        }

        self
    }

    /// Restrict the returned results to builds for the given update(s).
    /// Can be specified multiple times.
    pub fn updates(mut self, update: String) -> BuildQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
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
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

#[derive(Debug)]
struct BuildPageQuery {
    nvr: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<String>>,
    updates: Option<Vec<String>>,

    page: u32,
    rows_per_page: u32,
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
