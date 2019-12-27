use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::{BodhiServiceBuilder, CSRFQuery};

#[test]
fn deserialize() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize a new CSRF token
    bodhi.query(&CSRFQuery::new()).unwrap();
}
