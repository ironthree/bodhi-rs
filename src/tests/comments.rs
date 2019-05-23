use super::{TEST_RETRIES, TEST_TIMEOUT};
use crate::{BodhiService, Comment, CommentIDQuery, FEDORA_BODHI_URL};

#[test]
fn id_query_some() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let comment: Option<Comment> = CommentIDQuery::new(19999).query(&bodhi).unwrap();

    assert!(comment.is_some());
}

#[test]
fn id_query_none() {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES);

    let comment: Option<Comment> = CommentIDQuery::new(999999999).query(&bodhi).unwrap();

    assert!(comment.is_none());
}
