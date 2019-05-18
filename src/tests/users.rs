use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, UserQuery, FEDORA_BODHI_URL};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query and deserialize all users
    UserQuery::new().query(&bodhi).unwrap();
}
