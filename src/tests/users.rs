use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::{BodhiServiceBuilder, User, UserNameQuery};

// TODO: make sure the new serde_url_params code works as expected

#[test]
fn name_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let user: Option<User> = bodhi.query(&UserNameQuery::new("decathorpe")).unwrap();

    assert!(user.is_some());
}

#[test]
fn name_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let user: Option<User> = bodhi.query(&UserNameQuery::new("nobody")).unwrap();

    assert!(user.is_none());
}
