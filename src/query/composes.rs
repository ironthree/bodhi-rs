// ! The contents of this module can be used to query a bodhi instance for running composes.

use serde::Deserialize;

use crate::data::{Compose, ComposeRequest, FedoraRelease};
use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

// Use this for querying bodhi for a specific compose by its release and request. It will either
// return an `Ok(Some(Compose))` matching the specified values, return `Ok(None)` if it doesn't
// currently exist, or return an `Err(QueryError)` if another error occurred.
//
// ```
// # use bodhi::{BodhiServiceBuilder, ComposeReleaseRequestQuery, FedoraRelease, ComposeRequest};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let compose = bodhi
//     .query(ComposeReleaseRequestQuery::new(
//         FedoraRelease::F31,
//         ComposeRequest::Stable,
//     ))
//     .unwrap();
// ```
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
    // This method is the only way to create a new
    // [`ComposeReleaseRequestQuery`](struct.ComposeReleaseRequestQuery.html) instance.
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

// This query can be used to fetch information about currently running composes from bodhi.
//
// ```
// # use bodhi::{BodhiServiceBuilder, ComposeQuery};
// let bodhi = BodhiServiceBuilder::default().build().unwrap();
//
// # #[cfg(feature = "online-tests")]
// let composes = bodhi.query(ComposeQuery::new()).unwrap();
// ```
//
// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/composes.html>
#[derive(Debug, Default)]
pub struct ComposeQuery {}

#[derive(Debug, Deserialize)]
pub struct ComposeListPage {
    composes: Vec<Compose>,
}

impl ComposeQuery {
    // This method creates a new [`ComposeQuery`](struct.ComposeQuery.html).
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
