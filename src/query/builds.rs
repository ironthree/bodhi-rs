//! The contents of this module can be used to query a bodhi instance about existing builds.
//!
//! The [`BuildNVRQuery`](struct.BuildNVRQuery.html) returns exactly one Build, if and only if a
//! [`Build`](../../data/struct.Build.html) with the given Name-Version-Release triple exists -
//! otherwise, it will return an error.
//!
//! The [`BuildQuery`](struct.BuildQuery.html) can be used to execute more complex queries -
//! querying builds of certain packages, builds for certain releases, or builds associated with a
//! given set of updates is possible.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::{QueryError, ServiceError};
use crate::service::DEFAULT_ROWS;
use crate::{BodhiService, Build, FedoraRelease, Query, SinglePageQuery};

/// Use this for querying bodhi for a specific build, by its NVR (Name-Version-Release) string. It
/// will either return an `Ok(Some(Build))` matching the specified NVR, return `Ok(None)` if it
/// doesn't exist, or return an `Err(QueryError)` if another error occurred.
///
/// ```
/// # use bodhi::{BuildNVRQuery, BodhiServiceBuilder};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let build = bodhi.query(BuildNVRQuery::new("rust-1.34.1-1.fc29")).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/builds.html#service-0>
#[derive(Debug)]
pub struct BuildNVRQuery<'a> {
    /// NVR of the build to query (Name-Version-Release format, without Epoch)
    nvr: &'a str,
}

impl<'a> BuildNVRQuery<'a> {
    /// This method is the only way to create a new [`BuildNVRQuery`](struct.BuildNVRQuery.html)
    /// instance.
    pub fn new(nvr: &'a str) -> Self {
        BuildNVRQuery { nvr }
    }
}

impl<'a> SinglePageQuery<Option<Build>> for BuildNVRQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/builds/{}", self.nvr))
    }

    fn parse(string: &str) -> Result<Option<Build>, QueryError> {
        let build: Build = serde_json::from_str(string)?;
        Ok(Some(build))
    }

    fn missing() -> Result<Option<Build>, QueryError> {
        Ok(None)
    }
}

impl<'a> Query<Option<Build>> for BuildNVRQuery<'a> {
    fn query(self, bodhi: &BodhiService) -> Result<Option<Build>, QueryError> {
        <Self as SinglePageQuery<Option<Build>>>::query(self, bodhi)
    }
}

/// Use this for querying bodhi about a set of builds with the given properties, which can be
/// specified with the builder pattern. Note that some options can be specified multiple times, and
/// builds will be returned if any criteria match. This is consistent with both the web interface
/// and REST API behavior.
///
/// ```
/// # use bodhi::{BuildQuery, FedoraRelease, BodhiServiceBuilder};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let builds = bodhi
///     .query(
///         BuildQuery::new()
///             .releases(FedoraRelease::F30)
///             .releases(FedoraRelease::F29)
///             .packages("rust"),
///     )
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/builds.html#service-1>
#[derive(Default)]
pub struct BuildQuery<'a> {
    /// NVR of the build to query (Name-Version-Release format, without Epoch)
    nvr: Option<&'a str>,
    /// list of packages to request builds for
    packages: Option<Vec<&'a str>>,
    /// list of releases to request builds for
    releases: Option<Vec<FedoraRelease>>,
    /// list of updates to request builds for
    updates: Option<Vec<&'a str>>,

    /// optional callback function for reporting progress
    callback: Option<Box<dyn Fn(u32, u32) -> () + 'a>>,
}

impl<'a> Debug for BuildQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "BuildQuery {{ nvr: {:?}, packages: {:?}, releases: {:?}, updates: {:?} }}",
            &self.nvr, &self.packages, &self.releases, &self.updates,
        )
    }
}

impl<'a> BuildQuery<'a> {
    /// This method returns a new [`BuildQuery`](struct.BuildQuery.html) with *no* filters set.
    pub fn new() -> Self {
        BuildQuery {
            nvr: None,
            packages: None,
            releases: None,
            updates: None,
            callback: None,
        }
    }

    /// Add a callback function for reporting back query progress for long-running queries.
    /// The function will be called with the current page and the total number of pages for
    /// paginated queries.
    pub fn callback(mut self, fun: impl Fn(u32, u32) -> () + 'a) -> Self {
        self.callback = Some(Box::new(fun));
        self
    }

    /// Restrict the returned results to builds with the given NVR. If this is the only required
    /// filter, consider using a [`BuildNVRQuery`](struct.BuildNVRQuery.html) instead.
    pub fn nvr(mut self, nvr: &'a str) -> Self {
        self.nvr = Some(nvr);
        self
    }

    /// Restrict the returned results to builds of the given package(s).
    ///
    /// Can be specified multiple times.
    pub fn packages(mut self, package: &'a str) -> Self {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict the returned results to builds for the given release(s).
    ///
    /// Can be specified multiple times.
    pub fn releases(mut self, release: FedoraRelease) -> Self {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec![release]),
        }

        self
    }

    /// Restrict the returned results to builds for the given update(s).
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
    fn query(self, bodhi: &BodhiService) -> Result<Vec<Build>, QueryError> {
        let mut builds: Vec<Build> = Vec::new();
        let mut page = 1;

        loop {
            let query = self.page_query(page, DEFAULT_ROWS);
            let result = query.query(bodhi)?;

            if let Some(fun) = &self.callback {
                fun(page, result.pages);
            }

            builds.extend(result.builds);
            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(builds)
    }

    fn page_query(&self, page: u32, rows_per_page: u32) -> BuildPageQuery {
        BuildPageQuery {
            nvr: self.nvr,
            packages: self.packages.as_ref(),
            releases: self.releases.as_ref(),
            updates: self.updates.as_ref(),
            page,
            rows_per_page,
        }
    }
}

impl<'a> Query<Vec<Build>> for BuildQuery<'a> {
    fn query(self, bodhi: &BodhiService) -> Result<Vec<Build>, QueryError> {
        BuildQuery::query(self, bodhi)
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

#[derive(Debug, Serialize)]
struct BuildPageQuery<'a> {
    nvr: Option<&'a str>,
    packages: Option<&'a Vec<&'a str>>,
    releases: Option<&'a Vec<FedoraRelease>>,
    updates: Option<&'a Vec<&'a str>>,
    page: u32,
    rows_per_page: u32,
}

impl<'a> SinglePageQuery<BuildListPage> for BuildPageQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/builds/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(string: &str) -> Result<BuildListPage, QueryError> {
        let build_page: BuildListPage = serde_json::from_str(string)?;
        Ok(build_page)
    }

    fn missing() -> Result<BuildListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
