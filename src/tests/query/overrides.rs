use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::data::{FedoraRelease, Override};
use crate::query::{OverrideNVRQuery, OverrideQuery};
use crate::service::BodhiServiceBuilder;

#[test]
#[cfg(feature = "slow_tests")]
fn deserialize_all() {
    let bodhi = BodhiServiceBuilder::staging()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query all overrides (this is *very very slow*)
    bodhi.query(&OverrideQuery::new()).unwrap();
}

#[test]
fn deserialize_f32() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F32)).unwrap();
}

#[test]
fn deserialize_f32c() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F32C))
        .unwrap();
}

#[test]
fn deserialize_f31() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F31)).unwrap();
}

#[test]
fn deserialize_f31c() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F31C))
        .unwrap();
}

#[test]
fn deserialize_f31f() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F31F))
        .unwrap();
}

#[test]
fn deserialize_f31m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F31M))
        .unwrap();
}

#[test]
fn deserialize_f30() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F30)).unwrap();
}

#[test]
fn deserialize_f30c() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F30C))
        .unwrap();
}

#[test]
fn deserialize_f30f() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F30F))
        .unwrap();
}

#[test]
fn deserialize_f30m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F30M))
        .unwrap();
}

#[test]
fn deserialize_f29() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F29)).unwrap();
}

#[test]
fn deserialize_f29c() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F29C))
        .unwrap();
}

#[test]
fn deserialize_f29f() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F29F))
        .unwrap();
}

#[test]
fn deserialize_f29m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F29M))
        .unwrap();
}

#[test]
fn deserialize_f28() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F28)).unwrap();
}

#[test]
fn deserialize_f28c() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F28C))
        .unwrap();
}

#[test]
fn deserialize_f28m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F28M))
        .unwrap();
}

#[test]
fn deserialize_f27() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F27)).unwrap();
}

#[test]
fn deserialize_f27m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::F27M))
        .unwrap();
}

#[test]
fn deserialize_f26() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F26)).unwrap();
}

#[test]
fn deserialize_f25() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F25)).unwrap();
}

#[test]
fn deserialize_f24() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F24)).unwrap();
}

#[test]
fn deserialize_f23() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F23)).unwrap();
}

#[test]
fn deserialize_f22() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F22)).unwrap();
}

#[test]
fn deserialize_f21() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F21)).unwrap();
}

#[test]
fn deserialize_epel8() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::EPEL8))
        .unwrap();
}

#[test]
fn deserialize_epel8m() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::EPEL8M))
        .unwrap();
}

#[test]
fn deserialize_epel7() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi
        .query(&OverrideQuery::new().releases(FedoraRelease::EPEL7))
        .unwrap();
}

#[test]
fn deserialize_epel6() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::EL6)).unwrap();
}

#[test]
fn deserialize_epel5() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only overrides for one release, and deserialize them
    bodhi.query(&OverrideQuery::new().releases(FedoraRelease::EL5)).unwrap();
}

#[test]
fn nvr_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let over_ride: Option<Override> = bodhi
        .query(&OverrideNVRQuery::new(String::from("rust-1.34.2-1.fc30")))
        .unwrap();

    assert!(over_ride.is_some());
}

#[test]
fn nvr_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let over_ride: Option<Override> = bodhi
        .query(&OverrideNVRQuery::new(String::from("syncthing-1.1.3-1.fc30")))
        .unwrap();

    assert!(over_ride.is_none());
}

#[test]
fn nvr_query_invalid() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let over_ride: Option<Override> = bodhi
        .query(&OverrideNVRQuery::new(String::from("this-doesnt-exist-1-1.fc30")))
        .unwrap();

    assert!(over_ride.is_none());
}
