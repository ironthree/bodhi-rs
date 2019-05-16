use super::{SERVER_URL, TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, ReleaseQuery};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    ReleaseQuery::new()
        .exclude_archived(false)
        .query(&bodhi)
        .unwrap();
}
