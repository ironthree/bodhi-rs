use super::{SERVER_URL, TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, OverrideQuery};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only overrides for currently active releases
    OverrideQuery::new()
        .releases(String::from("F30"))
        .query(&bodhi)
        .unwrap();
    OverrideQuery::new()
        .releases(String::from("F29"))
        .query(&bodhi)
        .unwrap();
}
