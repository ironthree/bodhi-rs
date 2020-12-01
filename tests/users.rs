#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::read_to_string;

use bodhi::User;

const JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/users.json");

#[cfg(feature = "data-tests")]
#[test]
fn users_dejson() {
    let users: Vec<User> = serde_json::from_str(&read_to_string(JSON).unwrap()).unwrap();

    for user in &users {
        if !user.extra.is_empty() {
            println!("{:#?}", user.extra);
        }

        assert!(user.extra.is_empty());
    }

    // check if an optional field is no longer present
    assert!(!users.iter().all(|u| u.avatar.is_none()));
    assert!(!users.iter().all(|u| u.email.is_none()));
    assert!(!users.iter().all(|u| u.openid.is_none()));
}
