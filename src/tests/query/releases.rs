use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::data::*;
use crate::query::*;
use crate::service::*;

#[test]
fn deserialize_all() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize all releases
    ReleaseQuery::new()
        .exclude_archived(false)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn name_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let release: Option<Release> = ReleaseNameQuery::new(String::from("F30"))
        .query(&bodhi)
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

    let release: Option<Release> = ReleaseNameQuery::new(String::from("X10"))
        .query(&bodhi)
        .unwrap();

    assert!(release.is_none());
}
