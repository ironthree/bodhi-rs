use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, Build, BuildNVRQuery, BuildQuery, FEDORA_BODHI_URL};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only builds for the most recent release, and deserialize them
    BuildQuery::new()
        .releases(String::from("F30"))
        .query(&bodhi)
        .unwrap();
}

#[test]
fn nvr_query_some() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let build: Option<Build> = BuildNVRQuery::new(String::from("rust-1.34.2-1.fc30"))
        .query(&bodhi)
        .unwrap();

    assert!(build.is_some());
}

#[test]
fn nvr_query_none() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let build: Option<Build> = BuildNVRQuery::new(String::from("this-doesnt-exist-1-1.fc30"))
        .query(&bodhi)
        .unwrap();

    assert!(build.is_none());
}
