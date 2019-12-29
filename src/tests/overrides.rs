use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::{BodhiServiceBuilder, Override, OverrideNVRQuery};

// TODO: make sure the new serde_url_params code works as expected

#[test]
fn nvr_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let over_ride: Option<Override> = bodhi.query(&OverrideNVRQuery::new("rust-1.34.2-1.fc30")).unwrap();

    assert!(over_ride.is_some());
}

#[test]
fn nvr_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let over_ride: Option<Override> = bodhi.query(&OverrideNVRQuery::new("syncthing-1.1.3-1.fc30")).unwrap();

    assert!(over_ride.is_none());
}

#[test]
fn nvr_query_invalid() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let over_ride: Option<Override> = bodhi
        .query(&OverrideNVRQuery::new("this-doesnt-exist-1-1.fc30"))
        .unwrap();

    assert!(over_ride.is_none());
}
