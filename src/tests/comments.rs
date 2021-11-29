use super::bodhi_init;

use crate::{Comment, CommentIDQuery, CommentQuery};

#[test]
fn query_sanity_packages() {
    let bodhi = bodhi_init();

    let rs_commis: Vec<Comment> = bodhi.query(CommentQuery::new().packages(vec!["rust"])).unwrap();
    let go_commis: Vec<Comment> = bodhi.query(CommentQuery::new().packages(vec!["golang"])).unwrap();

    let both_commis: Vec<Comment> = bodhi
        .query(CommentQuery::new().packages(vec!["rust", "golang"]))
        .unwrap();

    assert_eq!(both_commis.len(), rs_commis.len() + go_commis.len())
}

#[test]
fn query_sanity_updates() {
    let bodhi = bodhi_init();

    let commis_one: Vec<Comment> = bodhi
        .query(CommentQuery::new().updates(vec!["FEDORA-2019-cf87377f5f"]))
        .unwrap();
    let commis_two: Vec<Comment> = bodhi
        .query(CommentQuery::new().updates(vec!["FEDORA-2019-24c9d17287"]))
        .unwrap();

    let both_commis: Vec<Comment> = bodhi
        .query(CommentQuery::new().updates(vec!["FEDORA-2019-cf87377f5f", "FEDORA-2019-24c9d17287"]))
        .unwrap();

    assert_eq!(both_commis.len(), commis_one.len() + commis_two.len())
}

#[test]
fn query_sanity_users() {
    let bodhi = bodhi_init();

    let commis_one: Vec<Comment> = bodhi.query(CommentQuery::new().users(vec!["astra"])).unwrap();
    let commis_two: Vec<Comment> = bodhi.query(CommentQuery::new().users(vec!["cipherboy"])).unwrap();

    let both_commis: Vec<Comment> = bodhi
        .query(CommentQuery::new().users(vec!["astra", "cipherboy"]))
        .unwrap();

    assert_eq!(both_commis.len(), commis_one.len() + commis_two.len())
}

#[test]
fn id_query_some() {
    let bodhi = bodhi_init();

    let comment: Option<Comment> = bodhi.query(CommentIDQuery::new(19_999)).unwrap();

    assert!(comment.is_some());
}

#[test]
fn id_query_none() {
    let bodhi = bodhi_init();

    let comment: Option<Comment> = bodhi.query(CommentIDQuery::new(999_999_999)).unwrap();

    assert!(comment.is_none());
}
