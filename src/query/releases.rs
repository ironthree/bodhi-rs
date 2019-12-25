//! The contents of this module can be used to query a bodhi instance about existing releases.
//!
//! The [`ReleaseNameQuery`](struct.ReleaseNameQuery.html) returns exactly one
//! [`Release`](../../data/struct.Release.html), if and only if a `Release` with this name exists -
//! otherwise, it will return an error.
//!
//! The [`ReleaseQuery`](struct.ReleaseQuery.html) can be used to execute more complex queries, for
//! example filtering releases by status, or query the releases associated with a given set of
//! updates or packages.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::Release;
use crate::error::{QueryError, ServiceError};
use crate::query::{Query, SinglePageQuery};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi for a specific release by its name. It will either return an
/// `Ok(Some(Release))` matching the specified name, return `Ok(None)` if it doesn't exist, or
/// return an `Err(String)` if another error occurred.
///
/// ```
/// # use bodhi::BodhiServiceBuilder;
/// # use bodhi::data::FedoraRelease;
/// # use bodhi::query::ReleaseNameQuery;
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let release = bodhi.query(&ReleaseNameQuery::new(String::from("F30"))).unwrap();
///
/// let release = bodhi.query(&ReleaseNameQuery::new(FedoraRelease::F30.into())).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/releases.html#service-0>
#[derive(Debug)]
pub struct ReleaseNameQuery {
    name: String,
}

impl ReleaseNameQuery {
    /// This method is the only way to create a new
    /// [`ReleaseNameQuery`](struct.ReleaseNameQuery.html) instance.
    pub fn new(name: String) -> Self {
        ReleaseNameQuery { name }
    }
}

impl SinglePageQuery<Option<Release>> for ReleaseNameQuery {
    fn path(&self) -> String {
        format!("/releases/{}", self.name)
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        None
    }

    fn parse(string: String) -> Result<Option<Release>, QueryError> {
        let release: Release = serde_json::from_str(&string)?;
        Ok(Some(release))
    }

    fn missing() -> Result<Option<Release>, QueryError> {
        Ok(None)
    }
}

impl Query<Option<Release>> for ReleaseNameQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Option<Release>, QueryError> {
        <Self as SinglePageQuery<Option<Release>>>::query(self, bodhi)
    }
}

/// Use this for querying bodhi about a set of releases with the given properties, which can be
/// specified with the builder pattern. Note that some options can be specified multiple times, and
/// comments will be returned if any criteria match. This is consistent with both the web interface
/// and REST API behavior.
///
/// ```
/// # use bodhi::BodhiServiceBuilder;
/// # use bodhi::query::ReleaseQuery;
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let releases = bodhi.query(&ReleaseQuery::new().exclude_archived(true)).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/releases.html#service-1>
#[derive(Debug, Default)]

pub struct ReleaseQuery {
    exclude_archived: Option<bool>,
    ids: Option<Vec<String>>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    updates: Option<Vec<String>>,
}

impl ReleaseQuery {
    /// This method returns a new [`ReleaseQuery`](struct.ReleaseQuery.html) with *no* filters set.
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
    ///
    /// Can be specified multiple times.
    pub fn ids(mut self, id: String) -> Self {
        match &mut self.ids {
            Some(ids) => ids.push(id),
            None => self.ids = Some(vec![id]),
        }

        self
    }

    /// Restrict results to a release with the given name. If this is the only required filter,
    /// consider using a [`ReleaseNameQuery`](struct.ReleaseNameQuery.html) instead.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Restrict the returned results to releases containing the given package(s).
    ///
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> Self {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict the returned results to releases matching the given updates(s).
    ///
    /// Can be specified multiple times.
    pub fn updates(mut self, update: String) -> Self {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Release>, QueryError> {
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

impl Query<Vec<Release>> for ReleaseQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Release>, QueryError> {
        ReleaseQuery::query(self, bodhi)
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
}

impl SinglePageQuery<ReleaseListPage> for ReleasePageQuery {
    fn path(&self) -> String {
        String::from("/releases/")
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(exclude_archived) = self.exclude_archived {
            args.insert("exclude_archived", exclude_archived.to_string());
        };

        if let Some(ids) = &self.ids {
            args.insert("ids", ids.join(","));
        };

        if let Some(name) = &self.name {
            args.insert("name", name.to_owned());
        };

        if let Some(packages) = &self.packages {
            args.insert("packages", packages.join(","));
        };

        if let Some(updates) = &self.updates {
            args.insert("updates", updates.join(","));
        };

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        Some(args)
    }

    fn parse(string: String) -> Result<ReleaseListPage, QueryError> {
        let release_page: ReleaseListPage = serde_json::from_str(&string)?;
        Ok(release_page)
    }

    fn missing() -> Result<ReleaseListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
