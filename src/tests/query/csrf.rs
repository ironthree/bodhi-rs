use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::query::*;
use crate::service::*;

#[test]
fn deserialize() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize a new CSRF token
    CSRFQuery::new().query(&bodhi).unwrap();
}
