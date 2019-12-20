use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::data::Release;
use crate::query::{ReleaseNameQuery, ReleaseQuery};
use crate::service::BodhiServiceBuilder;

#[test]
fn deserialize_all() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize all releases
    bodhi
        .query(&ReleaseQuery::new().exclude_archived(false))
        .unwrap();
}

#[test]
fn name_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let release: Option<Release> = bodhi
        .query(&ReleaseNameQuery::new(String::from("F30")))
        .unwrap();

    assert!(release.is_some());
}

#[test]
fn name_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let release: Option<Release> = bodhi
        .query(&ReleaseNameQuery::new(String::from("X12")))
        .unwrap();

    assert!(release.is_none());
}
