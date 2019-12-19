use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::data::User;
use crate::query::{UserNameQuery, UserQuery};
use crate::service::BodhiServiceBuilder;

#[test]
#[ignore]
fn deserialize_all() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query and deserialize all users
    bodhi.query(&UserQuery::new()).unwrap();
}

#[test]
fn name_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let user: Option<User> = bodhi.query(
        &UserNameQuery::new(String::from("decathorpe"))
    ).unwrap();

    assert!(user.is_some());
}

#[test]
fn name_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let user: Option<User> = bodhi.query(
        &UserNameQuery::new(String::from("nobody"))
    ).unwrap();

    assert!(user.is_none());
}
