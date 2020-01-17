#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::read_to_string;

use bodhi::Package;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/packages.json");

#[cfg(feature = "data-tests")]
#[test]
fn packages_dejson() {
    let _: Vec<Package> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();
}
