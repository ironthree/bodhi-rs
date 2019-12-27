use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::{BodhiServiceBuilder, Release, ReleaseNameQuery};

// TODO: make sure the new serde_url_params code works as expected

#[test]
fn name_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let release: Option<Release> = bodhi.query(&ReleaseNameQuery::new(String::from("F30"))).unwrap();

    assert!(release.is_some());
}

#[test]
fn name_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let release: Option<Release> = bodhi.query(&ReleaseNameQuery::new(String::from("X12"))).unwrap();

    assert!(release.is_none());
}
