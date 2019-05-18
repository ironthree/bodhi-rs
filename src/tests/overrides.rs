use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, OverrideQuery, FEDORA_BODHI_URL};

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
