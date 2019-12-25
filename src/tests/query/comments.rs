use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::data::Comment;
use crate::query::{CommentIDQuery, CommentQuery};
use crate::service::BodhiServiceBuilder;

#[test]
#[ignore]
fn deserialize_all() {
    let bodhi = BodhiServiceBuilder::staging()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query all comments (this is *very very slow*)
    bodhi.query(&CommentQuery::new()).unwrap();
}

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
