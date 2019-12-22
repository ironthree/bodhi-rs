//! The contents of this module can be used to query a bodhi instance about existing builds.
//! However, since rawhide builds are not passing through bodhi (yet), this can only be used to
//! query bodhi about the builds it knows about: rpm, module, and flatpak builds for stable
//! releases, and container builds for all releases (including rawhide).
//!
//! The [`BuildNVRQuery`](struct.BuildNVRQuery.html) returns exactly one Build, if and only if a
//! [`Build`](../../data/struct.Build.html) with the given Name-Version-Release triple exists -
//! otherwise, it will return an error.
//!
//! The [`BuildQuery`](struct.BuildQuery.html) can be used to execute more complex queries -
//! querying builds of certain packages, builds for certain releases, or builds associated with a
//! given set of updates is possible.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{Build, FedoraRelease};
use crate::error::QueryError;
use crate::query::{Query, SinglePageQuery};
use crate::service::{BodhiService, ServiceError, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi for a specific build, by its NVR (Name-Version-Release) string. It
/// will either return an `Ok(Some(Build))` matching the specified NVR, return `Ok(None)` if it
/// doesn't exist, or return an `Err(QueryError)` if another error occurred.
///
/// ```
/// # use bodhi::BodhiServiceBuilder;
/// # use bodhi::query::BuildNVRQuery;
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let build = bodhi
///     .query(&BuildNVRQuery::new(String::from("rust-1.34.1-1.fc29")))
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/builds.html#service-0>
#[derive(Debug)]
pub struct BuildNVRQuery {
    /// NVR of the build to query (Name-Version-Release format, without Epoch)
    nvr: String,
}

impl BuildNVRQuery {
    /// This method is the only way to create a new [`BuildNVRQuery`](struct.BuildNVRQuery.html)
    /// instance.
    pub fn new(nvr: String) -> Self {
        BuildNVRQuery { nvr }
    }
}

impl SinglePageQuery<Option<Build>> for BuildNVRQuery {
    fn path(&self) -> String {
        format!("/builds/{}", self.nvr)
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        None
    }

    fn parse(string: String) -> Result<Option<Build>, QueryError> {
        let build: Build = serde_json::from_str(&string)?;
        Ok(Some(build))
    }

    fn missing() -> Result<Option<Build>, QueryError> {
        Ok(None)
    }
}

impl Query<Option<Build>> for BuildNVRQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Option<Build>, QueryError> {
        <Self as SinglePageQuery<Option<Build>>>::query(self, bodhi)
    }
}

/// Use this for querying bodhi about a set of builds with the given properties, which can be
/// specified with the builder pattern. Note that some options can be specified multiple times, and
/// builds will be returned if any criteria match. This is consistent with both the web interface
/// and REST API behavior.
///
/// ```
/// # use bodhi::BodhiServiceBuilder;
/// # use bodhi::data::FedoraRelease;
/// # use bodhi::query::BuildQuery;
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let builds = bodhi
///     .query(
///         &BuildQuery::new()
///             .releases(FedoraRelease::F30)
///             .releases(FedoraRelease::F29)
///             .packages(String::from("rust")),
///     )
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/builds.html#service-1>
#[derive(Debug, Default)]
pub struct BuildQuery {
    /// NVR of the build to query (Name-Version-Release format, without Epoch)
    nvr: Option<String>,
    /// list of packages to request builds for
    packages: Option<Vec<String>>,
    /// list of releases to request builds for
    releases: Option<Vec<String>>,
    /// list of updates to request builds for
    updates: Option<Vec<String>>,
}

impl BuildQuery {
    /// This method returns a new [`BuildQuery`](struct.BuildQuery.html) with *no* filters set.
    pub fn new() -> BuildQuery {
        BuildQuery {
            nvr: None,
            packages: None,
            releases: None,
            updates: None,
        }
    }

    /// Restrict the returned results to builds with the given NVR. If this is the only required
    /// filter, consider using a [`BuildNVRQuery`](struct.BuildNVRQuery.html) instead.
    pub fn nvr(mut self, nvr: String) -> Self {
        self.nvr = Some(nvr);
        self
    }

    /// Restrict the returned results to builds of the given package(s).
    ///
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> Self {
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
            Some(releases) => releases.push(release.into()),
            None => self.releases = Some(vec![release.into()]),
        }

        self
    }

    /// Restrict the returned results to builds for the given update(s).
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
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Build>, QueryError> {
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

impl Query<Vec<Build>> for BuildQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Build>, QueryError> {
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
}

impl SinglePageQuery<BuildListPage> for BuildPageQuery {
    fn path(&self) -> String {
        String::from("/builds/")
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(nvr) = &self.nvr {
            args.insert("nvr", nvr.to_owned());
        }

        if let Some(packages) = &self.packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(releases) = &self.releases {
            args.insert("releases", releases.join(","));
        }

        if let Some(updates) = &self.updates {
            args.insert("updates", updates.join(","));
        }

        args.insert("page", format!("{}", &self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        Some(args)
    }

    fn parse(string: String) -> Result<BuildListPage, QueryError> {
        let build_page: BuildListPage = serde_json::from_str(&string)?;
        Ok(build_page)
    }

    fn missing() -> Result<BuildListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
