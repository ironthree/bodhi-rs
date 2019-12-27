use std::fs::read_to_string;
use bodhi::data::Release;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/releases.json");

#[test]
fn releases_dejson() {
    let _: Vec<Release> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();
}

