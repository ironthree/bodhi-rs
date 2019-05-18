use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, BuildQuery, FEDORA_BODHI_URL};

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
