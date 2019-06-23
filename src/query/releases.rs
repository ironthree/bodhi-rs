//! query for releases (or *one* release by name)
//!
//! The contents of this module can be used to query a bodhi instance about
//! existing releases.
//!
//! The `ReleaseNameQuery` returns exactly one Release, if and only if a Release
//! with this name exists - otherwise, it will return an error.
//!
//! The `ReleaseQuery` can be used to execute more complex queries, for example
//! filtering releases by status, or query the releases associated with a
//! given set of updates or packages.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::Release;
use crate::error::{BodhiError, QueryError};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

use super::retry_query;

/// Use this for querying bodhi for a specific release by its name.
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::new(String::from(bodhi::FEDORA_BODHI_URL))
///     .build().unwrap();
///
/// let release = bodhi::query::ReleaseNameQuery::new(String::from("F30"))
///     .query(&bodhi).unwrap();
///
/// let release = bodhi::query::ReleaseNameQuery::new(bodhi::data::FedoraRelease::F30.into())
///     .query(&bodhi).unwrap();
/// ```
#[derive(Debug)]
pub struct ReleaseNameQuery {
    name: String,
}

impl ReleaseNameQuery {
    /// This method is the only way to create a new `ReleaseNameQuery` instance.
    pub fn new(name: String) -> Self {
        ReleaseNameQuery { name }
    }

    /// This method will query the remote bodhi instance for the requested release by name,
    /// and will either return an `Ok(Some(Release))` matching the specified name,
    /// return `Ok(None)` if it doesn't exist, or return an `Err(String)`
    /// if another error occurred.
    pub fn query(self, bodhi: &BodhiService) -> Result<Option<Release>, QueryError> {
        let path = format!("/releases/{}", self.name);

        let mut response = bodhi.get(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let result = response.text()?;
            let release: Release = serde_json::from_str(&result)?;

            Ok(Some(release))
        } else {
            if status == 404 {
                // bodhi query successful, but release not found
                Ok(None)
            } else {
                // other server-side error
                let result = response.text()?;
                let error: BodhiError = serde_json::from_str(&result)?;

                Err(QueryError::BodhiError { error })
            }
        }
    }
}

/// Use this for querying bodhi about a set of releases with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and comments will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::service::BodhiServiceBuilder::new(String::from(bodhi::data::FEDORA_BODHI_URL))
///     .build().unwrap();
///
/// let releases = bodhi::query::ReleaseQuery::new()
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
    pub fn new() -> Self {
        ReleaseQuery {
            exclude_archived: None,
            ids: None,
            name: None,
            packages: None,
            updates: None,
        }
    }

    /// Restrict the returned results to (not) archived releases.
    pub fn exclude_archived(mut self, exclude_archived: bool) -> Self {
        self.exclude_archived = Some(exclude_archived);
        self
    }

    /// Restrict results to releases with the given ID.
    /// Can be specified multiple times.
    pub fn ids(mut self, id: String) -> Self {
        match &mut self.ids {
            Some(ids) => ids.push(id),
            None => self.ids = Some(vec![id]),
        }

        self
    }

    /// Restrict results to releases with the given name.
    /// If this is the only required filter, consider using a `ReleaseNameQuery` instead.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Restrict the returned results to releases containing the given package(s).
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> Self {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict the returned results to releases matching the given updates(s).
    /// Can be specified multiple times.
    pub fn updates(mut self, update: String) -> Self {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Release>, QueryError> {
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

    fn query(self, bodhi: &BodhiService) -> Result<ReleaseListPage, QueryError> {
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

        let result = retry_query(bodhi, &path, args)?;
        let releases: ReleaseListPage = serde_json::from_str(&result)?;

        Ok(releases)
    }
}
