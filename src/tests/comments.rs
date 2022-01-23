use super::bodhi_init;

use crate::error::QueryError;
use crate::{Comment, CommentIDQuery, CommentQuery};

#[tokio::test]
async fn query_sanity_packages() {
    let bodhi = bodhi_init().await;

    let rs_commis: Vec<Comment> = bodhi
        .paginated_request(&CommentQuery::new().packages(&["rust"]))
        .await
        .unwrap();
    let go_commis: Vec<Comment> = bodhi
        .paginated_request(&CommentQuery::new().packages(&["golang"]))
        .await
        .unwrap();

    let both_commis: Vec<Comment> = bodhi
        .paginated_request(&CommentQuery::new().packages(&["rust", "golang"]))
        .await
        .unwrap();

    assert_eq!(both_commis.len(), rs_commis.len() + go_commis.len())
}

#[tokio::test]
async fn query_sanity_updates() {
    let bodhi = bodhi_init().await;

    let commis_one: Vec<Comment> = bodhi
        .paginated_request(&CommentQuery::new().updates(&["FEDORA-2019-cf87377f5f"]))
        .await
        .unwrap();
    let commis_two: Vec<Comment> = bodhi
        .paginated_request(&CommentQuery::new().updates(&["FEDORA-2019-24c9d17287"]))
        .await
        .unwrap();

    let both_commis: Vec<Comment> = bodhi
        .paginated_request(&CommentQuery::new().updates(&["FEDORA-2019-cf87377f5f", "FEDORA-2019-24c9d17287"]))
        .await
        .unwrap();

    assert_eq!(both_commis.len(), commis_one.len() + commis_two.len())
}

#[tokio::test]
async fn query_sanity_users() {
    let bodhi = bodhi_init().await;

    let commis_one: Vec<Comment> = bodhi
        .paginated_request(&CommentQuery::new().users(&["astra"]))
        .await
        .unwrap();
    let commis_two: Vec<Comment> = bodhi
        .paginated_request(&CommentQuery::new().users(&["cipherboy"]))
        .await
        .unwrap();

    let both_commis: Vec<Comment> = bodhi
        .paginated_request(&CommentQuery::new().users(&["astra", "cipherboy"]))
        .await
        .unwrap();

    assert_eq!(both_commis.len(), commis_one.len() + commis_two.len())
}

#[tokio::test]
async fn id_query_ok() {
    let bodhi = bodhi_init().await;

    let comment = bodhi.request(&CommentIDQuery::new(19_999)).await;

    assert!(comment.is_ok());
}

#[tokio::test]
async fn id_query_err() {
    let bodhi = bodhi_init().await;

    let comment = bodhi.request(&CommentIDQuery::new(999_999_999)).await;

    assert!(matches!(comment, Err(QueryError::NotFound)));
}
