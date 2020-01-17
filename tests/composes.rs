#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::read_to_string;

use bodhi::Compose;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/composes.json");

#[cfg(feature = "data-tests")]
#[test]
fn composes_dejson() {
    let _: Vec<Compose> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();
}
