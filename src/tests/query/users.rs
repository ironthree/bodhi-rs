use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::data::*;
use crate::query::*;
use crate::service::*;

#[test]
fn deserialize_all() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize all users
    UserQuery::new().query(&bodhi).unwrap();
}

#[test]
fn name_query_some() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let user: Option<User> = UserNameQuery::new(String::from("decathorpe"))
        .query(&bodhi)
        .unwrap();

    assert!(user.is_some());
}

#[test]
fn name_query_none() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let user: Option<User> = UserNameQuery::new(String::from("nobody"))
        .query(&bodhi)
        .unwrap();

    assert!(user.is_none());
}
