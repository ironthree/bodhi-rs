#[cfg(feature = "slow_tests")]
use super::{TEST_RETRIES, TEST_TIMEOUT};

#[cfg(feature = "slow_tests")]
use crate::query::PackageQuery;
#[cfg(feature = "slow_tests")]
use crate::service::BodhiServiceBuilder;

#[test]
#[cfg(feature = "slow_tests")]
fn deserialize_all() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize all packages
    bodhi.query(&PackageQuery::new()).unwrap();
}
