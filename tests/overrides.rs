#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::read_to_string;

use bodhi::Override;

const JSON_F36: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f36.json");
const JSON_F36C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f36c.json");
const JSON_F35: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f35.json");
const JSON_F35C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f35c.json");
const JSON_F35F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f35f.json");
const JSON_F35M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f35m.json");
const JSON_F34: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f34.json");
const JSON_F34C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f34c.json");
const JSON_F34F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f34f.json");
const JSON_F34M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f34m.json");
const JSON_F33: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f33.json");
const JSON_F33C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f33c.json");
const JSON_F33F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f33f.json");
const JSON_F33M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f33m.json");
const JSON_F32: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f32.json");
const JSON_F32C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f32c.json");
const JSON_F32F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f32f.json");
const JSON_F32M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f32m.json");
const JSON_F31: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f31.json");
const JSON_F31C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f31c.json");
const JSON_F31F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f31f.json");
const JSON_F31M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f31m.json");
const JSON_F30: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f30.json");
const JSON_F30C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f30c.json");
const JSON_F30F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f30f.json");
const JSON_F30M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f30m.json");
const JSON_F29: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f29.json");
const JSON_F29C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f29c.json");
const JSON_F29F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f29f.json");
const JSON_F29M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f29m.json");
const JSON_F28: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f28.json");
const JSON_F28C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f28c.json");
const JSON_F28M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f28m.json");
const JSON_F27: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f27.json");
const JSON_F27M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f27m.json");
const JSON_F26: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f26.json");
const JSON_F25: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f25.json");
const JSON_F24: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f24.json");
const JSON_F23: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f23.json");
const JSON_F22: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f22.json");
const JSON_F21: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_f21.json");
const JSON_EPEL9: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_epel9.json");
const JSON_EPEL9N: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_epel9n.json");
const JSON_EPEL8: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_epel8.json");
const JSON_EPEL8M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_epel8m.json");
const JSON_EPEL8N: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_epel8n.json");
const JSON_EPEL7: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_epel7.json");
const JSON_EL6: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_el6.json");
const JSON_EL5: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_el5.json");
const JSON_ELN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/overrides_eln.json");


#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f36() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F36).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f36c() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F36C).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f35() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F35).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f35c() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F35C).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f35f() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F35F).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f35m() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F35M).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f34() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F34).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f34c() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F34C).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f34f() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F34F).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f34m() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F34M).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f33() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F33).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f33c() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F33C).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f33f() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F33F).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f33m() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F33M).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f32() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F32).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f32c() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F32C).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f32f() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F32F).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f32m() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F32M).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f31() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F31).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f31c() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F31C).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f31f() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F31F).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f31m() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F31M).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f30() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F30).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f30c() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F30C).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f30f() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F30F).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f30m() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F30M).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f29() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F29).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f29c() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F29C).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f29f() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F29F).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f29m() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F29M).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f28() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F28).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f28c() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F28C).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f28m() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F28M).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f27() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F27).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f27m() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F27M).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f26() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F26).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f25() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F25).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f24() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F24).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f23() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F23).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f22() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F22).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_f21() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_F21).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_epel9() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_EPEL9).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_epel9n() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_EPEL9N).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_epel8() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_EPEL8).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_epel8m() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_EPEL8M).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_epel8n() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_EPEL8N).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_epel7() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_EPEL7).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_el6() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_EL6).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_el5() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_EL5).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}

#[cfg(feature = "data-tests")]
#[test]
fn overrides_dejson_eln() {
    let os: Vec<Override> = serde_json::from_str(&read_to_string(JSON_ELN).unwrap()).unwrap();

    for o in &os {
        if !o.extra.is_empty() {
            println!("{:#?}", o.extra);
        }

        assert!(o.extra.is_empty());
    }

    // check if an optional field is no longer present
    if !os.is_empty() {
        assert!(!os.iter().all(|o| o.expired_date.is_none()));
    }
}
