//! The contents of this module can be used to query a bodhi instance about existing buildroot
//! overrides.
//!
//! The [`OverrideNVRQuery`](struct.OverrideNVRQuery.html) returns exactly one
//! [`Override`](../../data/struct.Override.html), if and only if an `Override` for the build with
//! this NVR exists - otherwise, it will return an error.
//!
//! The [`OverrideQuery`](struct.OverrideQuery.html) can be used to execute more complex queries,
//! for example filtering overrides by status, sets of overrides for certain packages, or overrides
//! filed by a given list of users.

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::{QueryError, ServiceError};
use crate::service::DEFAULT_ROWS;
use crate::{BodhiService, FedoraRelease, Override, Query, SinglePageQuery};

/// Use this for querying bodhi for a specific override, by its NVR (Name-Version-Release) string.
/// It will return either an `Ok(Some(Override))` matching the specified NVR, return `Ok(None)` if
/// it doesn't exist, or return an `Err(QueryError)` if another error occurred.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, OverrideNVRQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// # #[cfg(feature = "online-tests")]
/// let over_ride = bodhi.query(OverrideNVRQuery::new("wingpanel-2.2.1-1.fc28")).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/overrides.html#service-0>
#[derive(Debug)]
pub struct OverrideNVRQuery<'a> {
    nvr: &'a str,
}

#[derive(Debug, Deserialize)]
struct OverridePage {
    r#override: Override,
}

impl<'a> OverrideNVRQuery<'a> {
    /// This method is the only way to create a new
    /// [`OverrideNVRQuery`](struct.OverrideNVRQuery.html) instance.
    pub fn new(nvr: &'a str) -> Self {
        OverrideNVRQuery { nvr }
    }
}

impl<'a> SinglePageQuery<Option<Override>> for OverrideNVRQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/overrides/{}", self.nvr))
    }

    fn parse(string: &str) -> Result<Option<Override>, QueryError> {
        let override_page: OverridePage = serde_json::from_str(string)?;
        Ok(Some(override_page.r#override))
    }

    fn missing() -> Result<Option<Override>, QueryError> {
        Ok(None)
    }
}

#[async_trait::async_trait]
impl<'a> Query<'a, Option<Override>> for OverrideNVRQuery<'a> {
    async fn query(&'a self, bodhi: &'a BodhiService) -> Result<Option<Override>, QueryError> {
        <Self as SinglePageQuery<Option<Override>>>::query(self, bodhi)
    }
}

/// Use this for querying bodhi about a set of overrides with the given properties, which can be
/// specified with the builder pattern. Note that some options can be specified multiple times, and
/// overrides will be returned if any criteria match. This is consistent with both the web interface
/// and REST API behavior.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, FedoraRelease, OverrideQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// # #[cfg(feature = "online-tests")]
/// let overrides = bodhi
///     .query(
///         OverrideQuery::new()
///             .releases(vec![FedoraRelease::F29])
///             .users(vec!["decathorpe"]),
///     )
///     .unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/overrides.html#service-1>
#[derive(Default)]
pub struct OverrideQuery<'a> {
    builds: Option<Vec<&'a str>>,
    expired: Option<bool>,
    like: Option<&'a str>,
    packages: Option<Vec<&'a str>>,
    releases: Option<Vec<FedoraRelease>>,
    search: Option<&'a str>,
    users: Option<Vec<&'a str>>,

    /// optional callback function for reporting progress
    callback: Option<Box<dyn FnMut(u32, u32) + 'a>>,
}

impl<'a> Debug for OverrideQuery<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "OverrideQuery {{ builds: {:?}, expired: {:?}, like: {:?}, packages: {:?}, releases: {:?}, search: {:?}, users: {:?} }}",
            &self.builds, &self.expired, &self.like, &self.packages, &self.releases, &self.search, &self.users,
        )
    }
}

impl<'a> OverrideQuery<'a> {
    /// This method returns a new [`OverrideQuery`](struct.OverrideQuery.html) with *no* filters
    /// set.
    pub fn new() -> Self {
        OverrideQuery {
            builds: None,
            expired: None,
            like: None,
            packages: None,
            releases: None,
            search: None,
            users: None,
            callback: None,
        }
    }

    /// Add a callback function for reporting back query progress for long-running queries.
    /// The function will be called with the current page and the total number of pages for
    /// paginated queries.
    pub fn callback(mut self, fun: impl FnMut(u32, u32) + 'a) -> Self {
        self.callback = Some(Box::new(fun));
        self
    }

    /// Restrict the returned results to overrides for the given build(s).
    pub fn builds(mut self, builds: Vec<&'a str>) -> Self {
        self.builds = Some(builds);
        self
    }

    /// Restrict the returned results to (not) expired overrides.
    pub fn expired(mut self, expired: bool) -> Self {
        self.expired = Some(expired);
        self
    }

    /// Restrict search to overrides *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: &'a str) -> Self {
        self.like = Some(like);
        self
    }

    /// Restrict the returned results to overrides for the given package(s).
    pub fn packages(mut self, packages: Vec<&'a str>) -> Self {
        self.packages = Some(packages);
        self
    }

    /// Restrict the returned results to overrides for the given release(s).
    pub fn releases(mut self, releases: Vec<FedoraRelease>) -> Self {
        self.releases = Some(releases);
        self
    }

    /// Restrict search to overrides containing the given argument.
    pub fn search(mut self, search: &'a str) -> Self {
        self.search = Some(search);
        self
    }

    /// Restrict the returned results to overrides created by the given user(s).
    pub fn users(mut self, users: Vec<&'a str>) -> Self {
        self.users = Some(users);
        self
    }

    /// Query the remote bodhi instance with the given parameters.
    async fn query(mut self, bodhi: &BodhiService) -> Result<Vec<Override>, QueryError> {
        let mut overrides: Vec<Override> = Vec::new();
        let mut page = 1;

        // initial progress: 0 out of some
        if let Some(ref mut fun) = self.callback {
            fun(0, 1);
        }

        loop {
            let query = self.page_query(page, DEFAULT_ROWS);
            let result = query.query(bodhi).await?;

            if let Some(ref mut fun) = self.callback {
                fun(page, result.pages)
            }

            overrides.extend(result.r#overrides);
            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(overrides)
    }

    fn page_query(&self, page: u32, rows_per_page: u32) -> OverridePageQuery {
        OverridePageQuery {
            builds: self.builds.as_ref(),
            expired: self.expired,
            like: self.like,
            packages: self.packages.as_ref(),
            releases: self.releases.as_ref(),
            search: self.search.as_ref(),
            users: self.users.as_ref(),
            page,
            rows_per_page,
        }
    }
}

#[async_trait::async_trait]
impl<'a> Query<'a, Vec<Override>> for OverrideQuery<'a> {
    async fn query(&'a self, bodhi: &'a BodhiService) -> Result<Vec<Override>, QueryError> {
        OverrideQuery::query(self, bodhi)
    }
}

#[derive(Debug, Deserialize)]
struct OverrideListPage {
    overrides: Vec<Override>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

#[derive(Debug, Serialize)]
struct OverridePageQuery<'a> {
    builds: Option<&'a Vec<&'a str>>,
    expired: Option<bool>,
    like: Option<&'a str>,
    packages: Option<&'a Vec<&'a str>>,
    releases: Option<&'a Vec<FedoraRelease>>,
    search: Option<&'a &'a str>,
    #[serde(rename = "user")]
    users: Option<&'a Vec<&'a str>>,

    page: u32,
    rows_per_page: u32,
}

impl<'a> SinglePageQuery<OverrideListPage> for OverridePageQuery<'a> {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/overrides/?{}", serde_url_params::to_string(self)?))
    }

    fn parse(string: &str) -> Result<OverrideListPage, QueryError> {
        let override_page: OverrideListPage = serde_json::from_str(string)?;
        Ok(override_page)
    }

    fn missing() -> Result<OverrideListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
