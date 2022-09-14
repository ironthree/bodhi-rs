#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::read_to_string;

use bodhi::Update;

const JSON_F38: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f38.json");
const JSON_F38C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f38c.json");
const JSON_F37: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f37.json");
const JSON_F37C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f37c.json");
const JSON_F37F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f37f.json");
const JSON_F37M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f37m.json");
const JSON_F36: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f36.json");
const JSON_F36C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f36c.json");
const JSON_F36F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f36f.json");
const JSON_F36M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f36m.json");
const JSON_F35: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f35.json");
const JSON_F35C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f35c.json");
const JSON_F35F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f35f.json");
const JSON_F35M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f35m.json");
const JSON_F34: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f34.json");
const JSON_F34C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f34c.json");
const JSON_F34F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f34f.json");
const JSON_F34M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f34m.json");
const JSON_F33: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f33.json");
const JSON_F33C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f33c.json");
const JSON_F33F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f33f.json");
const JSON_F33M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f33m.json");
const JSON_F32: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f32.json");
const JSON_F32C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f32c.json");
const JSON_F32F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f32f.json");
const JSON_F32M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f32m.json");
const JSON_F31: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f31.json");
const JSON_F31C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f31c.json");
const JSON_F31F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f31f.json");
const JSON_F31M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f31m.json");
const JSON_F30: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f30.json");
const JSON_F30C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f30c.json");
const JSON_F30F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f30f.json");
const JSON_F30M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f30m.json");
const JSON_F29: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f29.json");
const JSON_F29C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f29c.json");
const JSON_F29F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f29f.json");
const JSON_F29M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f29m.json");
const JSON_F28: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f28.json");
const JSON_F28C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f28c.json");
const JSON_F28M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f28m.json");
const JSON_F27: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f27.json");
const JSON_F27M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f27m.json");
const JSON_F26: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f26.json");
const JSON_F25: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f25.json");
const JSON_F24: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f24.json");
const JSON_F23: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f23.json");
const JSON_F22: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f22.json");
const JSON_F21: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f21.json");
const JSON_EPEL9: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_epel9.json");
const JSON_EPEL9N: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_epel9n.json");
const JSON_EPEL8: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_epel8.json");
const JSON_EPEL8M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_epel8m.json");
const JSON_EPEL8N: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_epel8n.json");
const JSON_EPEL7: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_epel7.json");
const JSON_EL6: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_el6.json");
const JSON_EL5: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_el5.json");
const JSON_ELN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_eln.json");


#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f38() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F38).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f38c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F38C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f37() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F37).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        //assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f37c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F37C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        //assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f37f() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F37F).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f37m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F37M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f36() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F36).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f36c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F36C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        //assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f36f() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F36F).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f36m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F36M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        //assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        //assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f35() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F35).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f35c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F35C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        //assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f35f() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F35F).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f35m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F35M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f34() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F34).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f34c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F34C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f34f() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F34F).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f34m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F34M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f33() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F33).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f33c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F33C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f33f() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F33F).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f33m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F33M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f32() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F32).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f32c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F32C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f32f() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F32F).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f32m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F32M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f31() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F31).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f31c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F31C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f31f() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F31F).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f31m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F31M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f30() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F30).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f30c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F30C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f30f() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F30F).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f30m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F30M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f29() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F29).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f29c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F29C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f29f() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F29F).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f29m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F29M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f28() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F28).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f28c() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F28C).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f28m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F28M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f27() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F27).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f27m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F27M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        //assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        //assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f26() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F26).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f25() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F25).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f24() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F24).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f23() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F23).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f22() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F22).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f21() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F21).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_epel9() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EPEL9).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        //assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_epel9n() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EPEL9N).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_epel8() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EPEL8).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_epel8m() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EPEL8M).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_epel8n() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EPEL8N).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_epel7() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EPEL7).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_el6() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EL6).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_el5() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EL5).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_eln() {
    let updates: Vec<Update> = serde_json::from_str(&read_to_string(JSON_ELN).unwrap()).unwrap();

    for update in &updates {
        if !update.extra.is_empty() {
            println!("{:#?}", update.extra);
        }

        assert!(update.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !updates.is_empty() {
        assert!(!updates.iter().all(|u| u.comments.is_none()));
        assert!(!updates.iter().all(|u| u.content_type.is_none()));
        //assert!(!updates.iter().all(|u| u.date_modified.is_none()));
        assert!(!updates.iter().all(|u| u.date_pushed.is_none()));
        assert!(!updates.iter().all(|u| u.date_stable.is_none()));
        assert!(!updates.iter().all(|u| u.date_submitted.is_none()));
        assert!(!updates.iter().all(|u| u.date_testing.is_none()));
        assert!(!updates.iter().all(|u| u.karma.is_none()));
        assert!(!updates.iter().all(|u| u.requirements.is_none()));
        assert!(!updates.iter().all(|u| u.stable_days.is_none()));
        assert!(!updates.iter().all(|u| u.stable_karma.is_none()));
        assert!(!updates.iter().all(|u| u.test_cases.is_none()));
        assert!(!updates.iter().all(|u| u.unstable_karma.is_none()));
    }
}

