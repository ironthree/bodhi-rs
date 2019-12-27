use std::fs::read_to_string;

use bodhi::Compose;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/composes.json");

#[test]
fn composes_dejson() {
    let _: Vec<Compose> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();
}
