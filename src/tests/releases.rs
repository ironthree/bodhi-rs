use super::bodhi_init;

use crate::{Release, ReleaseNameQuery};

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
