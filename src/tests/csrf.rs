use super::{SERVER_URL, TEST_TIMEOUT, TEST_RETRIES};
use crate::{BodhiService, CSRFQuery};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    CSRFQuery::new().query(&bodhi).unwrap();
}