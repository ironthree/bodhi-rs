use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::data::Update;
use crate::query::UpdateIDQuery;
use crate::service::BodhiServiceBuilder;

// TODO: make sure the new serde_url_params code works as expected

#[test]
fn id_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let update: Option<Update> = bodhi
        .query(&UpdateIDQuery::new(String::from("FEDORA-2019-227c137c3f")))
        .unwrap();

    assert!(update.is_some());
}

#[test]
fn id_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let update: Option<Update> = bodhi.query(&UpdateIDQuery::new(String::from("NOPE"))).unwrap();

    assert!(update.is_none());
}
