use super::bodhi_init;

use crate::error::QueryError;
use crate::{Release, ReleaseNameQuery, ReleaseQuery};

#[tokio::test]
async fn query() {
    // This test makes sure that the FedoraRelease enum contains valid values for all fedora releases.
    // If this fails, then new enum variant(s) need to be added.

    let bodhi = bodhi_init().await;
    let _releases: Vec<Release> = bodhi.paginated_request(&ReleaseQuery::new()).await.unwrap();
}

#[tokio::test]
async fn name_query_ok() {
    let bodhi = bodhi_init().await;

    let release = bodhi.request(&ReleaseNameQuery::new("F30")).await;

    assert!(release.is_ok());
}

#[tokio::test]
async fn name_query_err() {
    let bodhi = bodhi_init().await;

    let release = bodhi.request(&ReleaseNameQuery::new("X12")).await;

    assert!(matches!(release, Err(QueryError::NotFound)));
}
