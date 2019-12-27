use std::fs::read_to_string;

use bodhi::User;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/users.json");

#[test]
fn users_dejson() {
    let _: Vec<User> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();
}
