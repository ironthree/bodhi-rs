use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::{BodhiServiceBuilder, Comment, CommentIDQuery};

// TODO: make sure the new serde_url_params code works as expected

#[test]
fn id_query_some() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let comment: Option<Comment> = bodhi.query(&CommentIDQuery::new(19_999)).unwrap();

    assert!(comment.is_some());
}

#[test]
fn id_query_none() {
    let bodhi = BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let comment: Option<Comment> = bodhi.query(&CommentIDQuery::new(999_999_999)).unwrap();

    assert!(comment.is_none());
}
