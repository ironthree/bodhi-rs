//! The contents of this module can be used to query a bodhi instance for running composes.

use serde::Deserialize;

use crate::error::{QueryError, ServiceError};
use crate::{BodhiService, Compose, ComposeRequest, FedoraRelease, Query, SinglePageQuery};

/// Use this for querying bodhi for a specific compose by its release and request. It will either
/// return an `Ok(Some(Compose))` matching the specified values, return `Ok(None)` if it doesn't
/// currently exist, or return an `Err(QueryError)` if another error occurred.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, CommentIDQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let comment = bodhi.query(CommentIDQuery::new(19999)).unwrap();
/// ```
#[derive(Debug)]
pub struct ComposeReleaseRequestQuery {
    release: FedoraRelease,
    request: ComposeRequest,
}

#[derive(Debug, Deserialize)]
struct ComposePage {
    compose: Compose,
}

impl ComposeReleaseRequestQuery {
    /// This method is the only way to create a new
    /// [`ComposeReleaseRequestQuery`](struct.ComposeReleaseRequestQuery.html) instance.
    pub fn new(release: FedoraRelease, request: ComposeRequest) -> Self {
        ComposeReleaseRequestQuery { release, request }
    }
}

impl SinglePageQuery<Option<Compose>> for ComposeReleaseRequestQuery {
    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/composes/{}/{}", self.release, self.request))
    }

    fn parse(string: &str) -> Result<Option<Compose>, QueryError> {
        let page: ComposePage = serde_json::from_str(string)?;
        Ok(Some(page.compose))
    }

    fn missing() -> Result<Option<Compose>, QueryError> {
        Ok(None)
    }
}

impl Query<Option<Compose>> for ComposeReleaseRequestQuery {
    fn query(self, bodhi: &BodhiService) -> Result<Option<Compose>, QueryError> {
        <Self as SinglePageQuery<Option<Compose>>>::query(self, bodhi)
    }
}

/// This query can be used to fetch information about currently running composes from bodhi.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, ComposeQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let composes = bodhi.query(ComposeQuery::new()).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/composes.html>
#[derive(Debug, Default)]
pub struct ComposeQuery {}

#[derive(Debug, Deserialize)]
struct ComposeListPage {
    composes: Vec<Compose>,
}

impl ComposeQuery {
    /// This method creates a new [`ComposeQuery`](struct.ComposeQuery.html).
    pub fn new() -> Self {
        ComposeQuery {}
    }
}

impl SinglePageQuery<Vec<Compose>> for ComposeQuery {
    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/composes/"))
    }

    fn parse(string: &str) -> Result<Vec<Compose>, QueryError> {
        let page: ComposeListPage = serde_json::from_str(string)?;
        Ok(page.composes)
    }

    fn missing() -> Result<Vec<Compose>, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}

impl Query<Vec<Compose>> for ComposeQuery {
    fn query(self, bodhi: &BodhiService) -> Result<Vec<Compose>, QueryError> {
        <Self as SinglePageQuery<Vec<Compose>>>::query(self, bodhi)
    }
}
