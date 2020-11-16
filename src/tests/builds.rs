use super::bodhi_init;

use crate::{Build, BuildNVRQuery, BuildQuery, FedoraRelease};

#[test]
fn query_sanity_packages() {
    let bodhi = bodhi_init();

    let rs_builds: Vec<Build> = bodhi.query(BuildQuery::new().packages(vec!["rust"])).unwrap();
    let go_builds: Vec<Build> = bodhi.query(BuildQuery::new().packages(vec!["golang"])).unwrap();

    let both_builds: Vec<Build> = bodhi.query(BuildQuery::new().packages(vec!["rust", "golang"])).unwrap();

    assert_eq!(both_builds.len(), rs_builds.len() + go_builds.len())
}

#[test]
fn query_sanity_releases() {
    let bodhi = bodhi_init();

    let f31c_builds: Vec<Build> = bodhi
        .query(BuildQuery::new().releases(vec![FedoraRelease::F31C]))
        .unwrap();
    let f30c_builds: Vec<Build> = bodhi
        .query(BuildQuery::new().releases(vec![FedoraRelease::F30C]))
        .unwrap();

    let both_builds: Vec<Build> = bodhi
        .query(BuildQuery::new().releases(vec![FedoraRelease::F31C, FedoraRelease::F30C]))
        .unwrap();

    assert_eq!(both_builds.len(), f31c_builds.len() + f30c_builds.len())
}

#[test]
fn query_sanity_updates() {
    let bodhi = bodhi_init();

    let builds_one: Vec<Build> = bodhi
        .query(BuildQuery::new().updates(vec!["FEDORA-2019-cf87377f5f"]))
        .unwrap();
    let builds_two: Vec<Build> = bodhi
        .query(BuildQuery::new().updates(vec!["FEDORA-2019-24c9d17287"]))
        .unwrap();

    let both_builds: Vec<Build> = bodhi
        .query(BuildQuery::new().updates(vec!["FEDORA-2019-cf87377f5f", "FEDORA-2019-24c9d17287"]))
        .unwrap();

    assert_eq!(both_builds.len(), builds_one.len() + builds_two.len())
}

#[test]
fn nvr_query_some() {
    let bodhi = bodhi_init();

    let build: Option<Build> = bodhi.query(BuildNVRQuery::new("rust-1.34.2-1.fc30")).unwrap();

    assert!(build.is_some());
}

#[test]
fn nvr_query_none() {
    let bodhi = bodhi_init();

    let build: Option<Build> = bodhi.query(BuildNVRQuery::new("this-doesnt-exist-1-1.fc30")).unwrap();

    assert!(build.is_none());
}
