use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, Release, ReleaseNameQuery, ReleaseQuery, FEDORA_BODHI_URL};

#[test]
fn deserialize_all() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query and deserialize all releases
    ReleaseQuery::new()
        .exclude_archived(false)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn name_query_some() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let build: Option<Release> = ReleaseNameQuery::new(String::from("F30"))
        .query(&bodhi)
        .unwrap();

    assert!(build.is_some());
}

#[test]
fn name_query_none() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let build: Option<Release> = ReleaseNameQuery::new(String::from("X10"))
        .query(&bodhi)
        .unwrap();

    assert!(build.is_none());
}
