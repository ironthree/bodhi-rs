use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::data::*;
use crate::query::*;
use crate::service::*;

#[test]
fn deserialize() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize a new CSRF token
    CSRFQuery::new().query(&bodhi).unwrap();
}
