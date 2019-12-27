use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::{BodhiServiceBuilder, ComposeQuery};

#[test]
fn deserialize() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize currently active composes
    bodhi.query(&ComposeQuery::new()).unwrap();
}
