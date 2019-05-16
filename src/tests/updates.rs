use super::{SERVER_URL, TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, UpdateQuery, UpdateStatus};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query only some updates from recent releases
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
