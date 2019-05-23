use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, Override, OverrideNVRQuery, OverrideQuery, FEDORA_BODHI_URL};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for currently active releases, and deserialize them
    OverrideQuery::new()
        .releases(String::from("F30"))
        .query(&bodhi)
        .unwrap();

    OverrideQuery::new()
        .releases(String::from("F29"))
        .query(&bodhi)
        .unwrap();
}

#[test]
fn nvr_query_some() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let over_ride: Option<Override> = OverrideNVRQuery::new(String::from("rust-1.34.2-1.fc30"))
        .query(&bodhi)
        .unwrap();

    assert!(over_ride.is_some());
}

#[test]
fn nvr_query_none() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let over_ride: Option<Override> = OverrideNVRQuery::new(String::from("syncthing-1.1.3-1.fc30"))
        .query(&bodhi)
        .unwrap();

    assert!(over_ride.is_none());
}

#[test]
fn nvr_query_invalid() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let over_ride: Option<Override> =
        OverrideNVRQuery::new(String::from("this-doesnt-exist-1-1.fc30"))
            .query(&bodhi)
            .unwrap();

    assert!(over_ride.is_none());
}
