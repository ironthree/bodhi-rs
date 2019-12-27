//! The contents of this module can be used to query a bodhi instance for running composes.

use serde::Deserialize;

use crate::error::{QueryError, ServiceError};
use crate::{BodhiService, Compose, Query, SinglePageQuery};

/// This query can be used to fetch information about currently running composes from bodhi.
///
/// ```
/// # use bodhi::{BodhiServiceBuilder, ComposeQuery};
/// let bodhi = BodhiServiceBuilder::default().build().unwrap();
///
/// let composes = bodhi.query(&ComposeQuery::new()).unwrap();
/// ```
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/composes.html>
#[derive(Debug, Default)]
pub struct ComposeQuery {}

#[derive(Debug, Deserialize)]
struct ComposePage {
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

    fn parse(string: String) -> Result<Vec<Compose>, QueryError> {
        let page: ComposePage = serde_json::from_str(&string)?;
        Ok(page.composes)
    }

    fn missing() -> Result<Vec<Compose>, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}

impl Query<Vec<Compose>> for ComposeQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Compose>, QueryError> {
        <Self as SinglePageQuery<Vec<Compose>>>::query(self, bodhi)
    }
}
