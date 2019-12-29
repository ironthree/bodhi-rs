use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::{BodhiServiceBuilder, Update, UpdateIDQuery};

// TODO: make sure the new serde_url_params code works as expected

#[test]
fn id_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let update: Option<Update> = bodhi.query(&UpdateIDQuery::new("FEDORA-2019-227c137c3f")).unwrap();

    assert!(update.is_some());
}

#[test]
fn id_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let update: Option<Update> = bodhi.query(&UpdateIDQuery::new("NOPE")).unwrap();

    assert!(update.is_none());
}
