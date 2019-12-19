use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::query::PackageQuery;
use crate::service::BodhiServiceBuilder;

#[test]
#[ignore]
fn deserialize_all() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize all packages
    bodhi.query(&PackageQuery::new()).unwrap();
}
