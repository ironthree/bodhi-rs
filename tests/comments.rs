use std::fs::read_to_string;

use bodhi::Comment;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/comments.json");

#[test]
fn comments_dejson() {
    let _: Vec<Comment> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();
}
