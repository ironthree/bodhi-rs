use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, User, UserNameQuery, UserQuery, FEDORA_BODHI_URL};

#[test]
fn deserialize() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    // query and deserialize all users
    UserQuery::new().query(&bodhi).unwrap();
}

#[test]
fn name_query_some() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let user: Option<User> = UserNameQuery::new(String::from("decathorpe"))
        .query(&bodhi)
        .unwrap();

    assert!(user.is_some());
}

#[test]
fn name_query_none() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let user: Option<User> = UserNameQuery::new(String::from("nobody"))
        .query(&bodhi)
        .unwrap();

    assert!(user.is_none());
}
