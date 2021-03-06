#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::read_to_string;

use bodhi::Comment;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/comments.json");

#[cfg(feature = "data-tests")]
#[test]
fn comments_dejson() {
    let comments: Vec<Comment> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();

    for comment in &comments {
        if !comment.extra.is_empty() {
            println!("{:#?}", comment.extra);
        }

        assert!(comment.extra.is_empty());
    }

    // check if an optional field is no longer present
    assert!(!comments.iter().all(|c| c.update.is_none()));
}
