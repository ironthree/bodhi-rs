use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, Stack, StackNameQuery, StackQuery, FEDORA_BODHI_URL};

#[test]
fn deserialize_all() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query and deserialize all stacks
    assert!(StackQuery::new().query(&bodhi).is_ok());;
}

#[test]
fn name_query_some() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let stack: Option<Stack> = StackNameQuery::new(String::from("SomeStack"))
        .query(&bodhi)
        .unwrap();

    assert!(stack.is_some());
}

#[test]
fn name_query_none() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let stack: Option<Stack> = StackNameQuery::new(String::from("NOPE"))
        .query(&bodhi)
        .unwrap();

    assert!(stack.is_none());
}
