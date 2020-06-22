#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::read_to_string;

use bodhi::Release;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/releases.json");

#[cfg(feature = "data-tests")]
#[test]
fn releases_dejson() {
    let releases: Vec<Release> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();

    for release in releases {
        if !release.extra.is_empty() {
            println!("{:#?}", release.extra);
        }

        assert!(release.extra.is_empty());
    }
}
