use super::{SERVER_URL, TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, StackQuery};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    StackQuery::new().query(&bodhi).unwrap();
}
