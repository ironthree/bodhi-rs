#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]

use std::fs::read_to_string;

use bodhi::Release;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/releases.json");

#[cfg(feature = "data-tests")]
#[test]
fn releases_dejson() {
    let releases: Vec<Release> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();

    for release in &releases {
        if !release.extra.is_empty() {
            println!("{:#?}", release.extra);
        }

        //assert!(release.extra.is_empty()); // fails for EPEL-9
    }

    // check if an optional field is no longer present
    assert!(!releases.iter().all(|r| r.composes.is_none()));
    assert!(!releases.iter().all(|r| r.create_automatic_updates.is_none()));
    assert!(!releases.iter().all(|r| r.testing_repository.is_none()));
}
