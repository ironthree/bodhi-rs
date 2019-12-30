use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::{BodhiServiceBuilder, ComposeQuery, ComposeReleaseRequestQuery, ComposeRequest, FedoraRelease};

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

#[test]
fn query() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize currently active composes
    bodhi
        .query(&ComposeReleaseRequestQuery::new(
            FedoraRelease::F31,
            ComposeRequest::Stable,
        ))
        .unwrap();
}
