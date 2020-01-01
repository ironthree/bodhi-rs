use super::bodhi_init;

use crate::{FedoraRelease, Override, OverrideNVRQuery, OverrideQuery};

#[test]
fn query_sanity_packages() {
    let bodhi = bodhi_init();

    let rs_overs: Vec<Override> = bodhi.query(&OverrideQuery::new().packages("rust")).unwrap();
    let go_overs: Vec<Override> = bodhi.query(&OverrideQuery::new().packages("golang")).unwrap();

    let both_overs: Vec<Override> = bodhi
        .query(&OverrideQuery::new().packages("rust").packages("golang"))
        .unwrap();

    assert_eq!(both_overs.len(), rs_overs.len() + go_overs.len())
}

#[test]
fn query_sanity_releases() {
    let bodhi = bodhi_init();

    let f31_overs: Vec<Override> = bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F31)).unwrap();
    let f32_overs: Vec<Override> = bodhi.query(&OverrideQuery::new().releases(FedoraRelease::F32)).unwrap();

    let both_overs: Vec<Override> = bodhi
        .query(
            &OverrideQuery::new()
                .releases(FedoraRelease::F31)
                .releases(FedoraRelease::F32),
        )
        .unwrap();

    assert_eq!(both_overs.len(), f31_overs.len() + f32_overs.len())
}

#[test]
fn query_sanity_users() {
    let bodhi = bodhi_init();

    let overs_one: Vec<Override> = bodhi.query(&OverrideQuery::new().users("gil")).unwrap();
    let overs_two: Vec<Override> = bodhi.query(&OverrideQuery::new().users("lef")).unwrap();

    let both_overs: Vec<Override> = bodhi.query(&OverrideQuery::new().users("gil").users("lef")).unwrap();

    assert_eq!(both_overs.len(), overs_one.len() + overs_two.len())
}

#[test]
fn nvr_query_some() {
    let bodhi = bodhi_init();

    let over_ride: Option<Override> = bodhi.query(&OverrideNVRQuery::new("rust-1.34.2-1.fc30")).unwrap();

    assert!(over_ride.is_some());
}

#[test]
fn nvr_query_none() {
    let bodhi = bodhi_init();

    let over_ride: Option<Override> = bodhi.query(&OverrideNVRQuery::new("syncthing-1.1.3-1.fc30")).unwrap();

    assert!(over_ride.is_none());
}

#[test]
fn nvr_query_invalid() {
    let bodhi = bodhi_init();

    let over_ride: Option<Override> = bodhi
        .query(&OverrideNVRQuery::new("this-doesnt-exist-1-1.fc30"))
        .unwrap();

    assert!(over_ride.is_none());
}
