use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Release};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi for a specific release by its name.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from(bodhi::FEDORA_BODHI_URL));
///
/// let comment = bodhi::ReleaseNameQuery::new(String::from("F30"))
///     .query(&bodhi).unwrap();
/// ```
#[derive(Debug)]
pub struct ReleaseNameQuery {
    name: String,
}

impl ReleaseNameQuery {
    /// This method is the only way to create a new `ReleaseNameQuery` instance.
    pub fn new(name: String) -> ReleaseNameQuery {
        ReleaseNameQuery { name }
    }

    /// This method will query the remote bodhi instance for the requested release by name,
    /// and will either return an `Ok(Release)` matching the specified name,
    /// or return an `Err(String)` if it doesn't exist, or if another error occurred.
    ///
    /// TODO: return `Result<Option<Release>, String>>` to distinguish "not found" from errors
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

/// Use this for querying bodhi about a set of releases with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and comments will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from(bodhi::FEDORA_BODHI_URL));
///
/// let releases = bodhi::ReleaseQuery::new()
///     .exclude_archived(true)
///     .query(&bodhi).unwrap();
/// ```
#[derive(Debug, Default)]
pub struct ReleaseQuery {
    exclude_archived: Option<bool>,
    ids: Option<Vec<String>>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    updates: Option<Vec<String>>,
}

impl ReleaseQuery {
    /// This method returns a new `ReleaseQuery` with *no* filters set.
    pub fn new() -> ReleaseQuery {
        ReleaseQuery {
            exclude_archived: None,
            ids: None,
            name: None,
            packages: None,
            updates: None,
        }
    }

    /// Restrict the returned results to (not) archived releases.
    pub fn exclude_archived(mut self, exclude_archived: bool) -> ReleaseQuery {
        self.exclude_archived = Some(exclude_archived);
        self
    }

    /// Restrict results to releases with the given ID.
    /// Can be specified multiple times.
    pub fn ids(mut self, id: String) -> ReleaseQuery {
        match &mut self.ids {
            Some(ids) => ids.push(id),
            None => self.ids = Some(vec![id]),
        }

        self
    }

    /// Restrict results to releases with the given name.
    /// If this is the only required filter, consider using a `ReleaseNameQuery` instead.
    pub fn name(mut self, name: String) -> ReleaseQuery {
        self.name = Some(name);
        self
    }

    /// Restrict the returned results to releases containing the given package(s).
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> ReleaseQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict the returned results to releases matching the given updates(s).
    /// Can be specified multiple times.
    pub fn updates(mut self, update: String) -> ReleaseQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
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
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

#[derive(Debug)]
struct ReleasePageQuery {
    exclude_archived: Option<bool>,
    ids: Option<Vec<String>>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    updates: Option<Vec<String>>,

    page: u32,
    rows_per_page: u32,
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
