use serde::Deserialize;

use crate::data::{Compose, ComposeRequest, FedoraRelease};
use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

/// data type encapsulating parameters for querying for a [`Compose`] by release and request type
///
/// If no compose with these properties is currently running, a [`QueryError::NotFound`] error is
/// returned for the query.
///
/// ```
/// use bodhi::{ComposeReleaseRequestQuery, ComposeRequest, ContentType, FedoraRelease};
///
/// let query = ComposeReleaseRequestQuery::new(
///     &FedoraRelease::fedora(34, ContentType::RPM).unwrap(),
///     ComposeRequest::Stable,
/// );
/// // let compose = bodhi.request(&query).unwrap();
/// ```
#[derive(Debug)]
pub struct ComposeReleaseRequestQuery<'a> {
    release: &'a FedoraRelease,
    request: ComposeRequest,
}

#[derive(Debug, Deserialize)]
pub struct ComposePage {
    compose: Compose,
}

impl<'a> ComposeReleaseRequestQuery<'a> {
    /// constructor for [`ComposeReleaseRequestQuery`] from Fedora release and request type
    pub fn new(release: &'a FedoraRelease, request: ComposeRequest) -> Self {
        ComposeReleaseRequestQuery { release, request }
    }
}

impl<'a> SingleRequest<ComposePage, Compose> for ComposeReleaseRequestQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/composes/{}/{}", self.release, self.request))
    }

    fn parse(&self, string: &str) -> Result<ComposePage, QueryError> {
        let page: ComposePage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: ComposePage) -> Compose {
        page.compose
    }
}


/// data type encapsulating (no) parameters for querying currently running [`Compose`]s
///
/// ```
/// use bodhi::ComposeQuery;
///
/// let query = ComposeQuery::new();
/// // let composes = bodhi.request(&query).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/composes.html>
#[derive(Debug, Default)]
pub struct ComposeQuery {}

#[derive(Debug, Deserialize)]
pub struct ComposeListPage {
    composes: Vec<Compose>,
}

impl ComposeQuery {
    /// constructor for [`ComposeQuery`] (no mandatory or optional parameters)
    pub fn new() -> Self {
        Self::default()
    }
}

impl SingleRequest<ComposeListPage, Vec<Compose>> for ComposeQuery {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/composes/"))
    }

    fn parse(&self, string: &str) -> Result<ComposeListPage, QueryError> {
        let page: ComposeListPage = serde_json::from_str(string)?;
        Ok(page)
    }

    fn extract(&self, page: ComposeListPage) -> Vec<Compose> {
        page.composes
    }
}
