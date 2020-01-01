use super::bodhi_init;

use crate::{ComposeQuery, ComposeReleaseRequestQuery, ComposeRequest, FedoraRelease};

#[test]
fn deserialize() {
    let bodhi = bodhi_init();

    // query and deserialize currently active composes
    bodhi.query(&ComposeQuery::new()).unwrap();
}

#[test]
fn query() {
    let bodhi = bodhi_init();

    // query and deserialize currently active composes
    bodhi
        .query(&ComposeReleaseRequestQuery::new(
            FedoraRelease::F31,
            ComposeRequest::Stable,
        ))
        .unwrap();
}
