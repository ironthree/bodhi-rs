//! The contents of this module can be used to query a bodhi instance about existing releases.
//!
//! The [`ReleaseNameQuery`](struct.ReleaseNameQuery.html) returns exactly one
//! [`Release`](../../data/struct.Release.html), if and only if a `Release` with this name exists -
//! otherwise, it will return an error.
//!
//! The [`ReleaseQuery`](struct.ReleaseQuery.html) can be used to execute more complex queries, for
//! example filtering releases by status, or query the releases associated with a given set of
//! updates or packages.

use serde::{Deserialize, Serialize};

use crate::error::{QueryError, ServiceError};
use crate::service::DEFAULT_ROWS;
use crate::{BodhiService, Query, Release, SinglePageQuery};

/// Use this for querying bodhi for a specific release by its name. It will either return an
/// `Ok(Some(Release))` matching the specified name, return `Ok(None)` if it doesn't exist, or
/// return an `Err(QueryError)` if another error occurred.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, FedoraRelease, ReleaseNameQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let release = bodhi.query(&ReleaseNameQuery::new("F30")).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/releases.html#service-0>
#[derive(Debug)]
pub struct ReleaseNameQuery<'a> {
    name: &'a str,
}

impl<'a> ReleaseNameQuery<'a> {
    /// This method is the only way to create a new
    /// [`ReleaseNameQuery`](struct.ReleaseNameQuery.html) instance.
    pub fn new(name: &'a str) -> Self {
        ReleaseNameQuery { name }
    }
}

impl<'a> SinglePageQuery<Option<Release>> for ReleaseNameQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/releases/{}", self.name))
    }

    fn parse(string: &str) -> Result<Option<Release>, QueryError> {
        let release: Release = serde_json::from_str(string)?;
        Ok(Some(release))
    }

    fn missing() -> Result<Option<Release>, QueryError> {
        Ok(None)
    }
}

impl<'a> Query<Option<Release>> for ReleaseNameQuery<'a> {
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
/// # use bodhi::{BodhiServiceBuilder, ReleaseQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let releases = bodhi.query(&ReleaseQuery::new().exclude_archived(true)).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/releases.html#service-1>
#[derive(Debug, Default)]

pub struct ReleaseQuery<'a> {
    exclude_archived: Option<bool>,
    ids: Option<Vec<&'a str>>,
    name: Option<&'a str>,
    packages: Option<Vec<&'a str>>,
    updates: Option<Vec<&'a str>>,
}

impl<'a> ReleaseQuery<'a> {
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
    pub fn ids(mut self, id: &'a str) -> Self {
        match &mut self.ids {
            Some(ids) => ids.push(id),
            None => self.ids = Some(vec![id]),
        }

        self
    }

    /// Restrict results to a release with the given name. If this is the only required filter,
    /// consider using a [`ReleaseNameQuery`](struct.ReleaseNameQuery.html) instead.
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// Restrict the returned results to releases containing the given package(s).
    ///
    /// Can be specified multiple times.
    pub fn packages(mut self, package: &'a str) -> Self {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict the returned results to releases matching the given updates(s).
    ///
    /// Can be specified multiple times.
    pub fn updates(mut self, update: &'a str) -> Self {
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
            let query = self.page_query(page, DEFAULT_ROWS);
            let result = query.query(bodhi)?;

            overrides.extend(result.releases);
            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(overrides)
    }

    fn page_query(&self, page: u32, rows_per_page: u32) -> ReleasePageQuery {
        ReleasePageQuery {
            exclude_archived: self.exclude_archived,
            ids: self.ids.as_ref(),
            name: self.name.as_ref(),
            packages: self.packages.as_ref(),
            updates: self.updates.as_ref(),
            page,
            rows_per_page,
        }
    }
}

impl<'a> Query<Vec<Release>> for ReleaseQuery<'a> {
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

#[derive(Debug, Serialize)]
struct ReleasePageQuery<'a> {
    exclude_archived: Option<bool>,
    ids: Option<&'a Vec<&'a str>>,
    name: Option<&'a &'a str>,
    packages: Option<&'a Vec<&'a str>>,
    updates: Option<&'a Vec<&'a str>>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> SinglePageQuery<ReleaseListPage> for ReleasePageQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/releases/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(string: &str) -> Result<ReleaseListPage, QueryError> {
        let release_page: ReleaseListPage = serde_json::from_str(string)?;
        Ok(release_page)
    }

    fn missing() -> Result<ReleaseListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
