use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::{BodhiServiceBuilder, Build, BuildNVRQuery};

// TODO: make sure the new serde_url_params code works as expected

#[test]
fn nvr_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let build: Option<Build> = bodhi
        .query(&BuildNVRQuery::new(String::from("rust-1.34.2-1.fc30")))
        .unwrap();

    assert!(build.is_some());
}

#[test]
fn nvr_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let build: Option<Build> = bodhi
        .query(&BuildNVRQuery::new(String::from("this-doesnt-exist-1-1.fc30")))
        .unwrap();

    assert!(build.is_none());
}
