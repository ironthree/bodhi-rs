use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::query::*;
use crate::service::*;

#[test]
fn deserialize_all() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize all packages
    PackageQuery::new().query(&bodhi).unwrap();
}
