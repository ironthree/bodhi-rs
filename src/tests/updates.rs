use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, UpdateQuery, UpdateStatus, FEDORA_BODHI_URL};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only *some* updates from recent releases, and deserialize them
    UpdateQuery::new()
        .releases(String::from("F30"))
        .critpath(true)
        .query(&bodhi)
        .unwrap();

    UpdateQuery::new()
        .releases(String::from("F29"))
        .status(UpdateStatus::Testing)
        .query(&bodhi)
        .unwrap();
}
