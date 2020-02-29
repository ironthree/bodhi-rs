use super::bodhi_init;

use crate::{Release, ReleaseNameQuery, ReleaseQuery};

#[test]
fn query() {
    // This test makes sure that the FedoraRelease enum contains valid values for all fedora releases.
    // If this fails, then new enum variant(s) need to be added.

    let bodhi = bodhi_init();
    let _releases: Vec<Release> = bodhi.query(ReleaseQuery::new()).unwrap();
}

#[test]
fn name_query_some() {
    let bodhi = bodhi_init();

    let release: Option<Release> = bodhi.query(ReleaseNameQuery::new("F30")).unwrap();

    assert!(release.is_some());
}

#[test]
fn name_query_none() {
    let bodhi = bodhi_init();

    let release: Option<Release> = bodhi.query(ReleaseNameQuery::new("X12")).unwrap();

    assert!(release.is_none());
}
