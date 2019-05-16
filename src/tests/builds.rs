use super::{SERVER_URL, TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, BuildQuery};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only builds for the most recent release
    BuildQuery::new()
        .releases(String::from("F30"))
        .query(&bodhi)
        .unwrap();
}
