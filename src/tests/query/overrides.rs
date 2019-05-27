use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, Override, OverrideNVRQuery, OverrideQuery, FEDORA_BODHI_URL};

#[test]
fn deserialize_f31c() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F31C"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f30() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F30"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f30c() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F30C"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f30f() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F30F"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f30m() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F30M"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f29() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F29"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f29c() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F29C"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f29f() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F29F"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f29m() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F29M"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f28() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F28"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f28c() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F28C"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f28m() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F28M"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f27() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F27"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f27m() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F27M"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f26() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F26"))
        .query(&bodhi)
        .is_ok());
}

#[test]
fn deserialize_f25() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for one release, and deserialize them
    assert!(OverrideQuery::new()
        .releases(String::from("F25"))
        .query(&bodhi)
        .is_ok());
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
