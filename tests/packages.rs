#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::read_to_string;

use bodhi::Package;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/packages.json");

#[cfg(feature = "data-tests")]
#[test]
fn packages_dejson() {
    let packages: Vec<Package> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();

    for package in &packages {
        if !package.extra.is_empty() {
            println!("{:#?}", package.extra);
        }

        assert!(package.extra.is_empty());
    }
}
