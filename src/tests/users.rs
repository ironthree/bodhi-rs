use super::bodhi_init;

use crate::error::QueryError;
use crate::{User, UserNameQuery, UserQuery};

#[tokio::test]
async fn query_sanity_updates() {
    let bodhi = bodhi_init().await;

    let users_one: Vec<User> = bodhi
        .paginated_request(&UserQuery::new().updates(vec!["FEDORA-2019-ac2a21ff07"]))
        .await
        .unwrap();
    let users_two: Vec<User> = bodhi
        .paginated_request(&UserQuery::new().updates(vec!["FEDORA-2019-ac3dc27f26"]))
        .await
        .unwrap();

    let users_both: Vec<User> = bodhi
        .paginated_request(&UserQuery::new().updates(vec!["FEDORA-2019-ac2a21ff07", "FEDORA-2019-ac3dc27f26"]))
        .await
        .unwrap();

    assert_eq!(users_both.len(), users_one.len() + users_two.len())
}

#[tokio::test]
async fn name_query_ok() {
    let bodhi = bodhi_init().await;

    let user = bodhi.request(&UserNameQuery::new("decathorpe")).await;

    assert!(user.is_ok());
}

#[tokio::test]
async fn name_query_err() {
    let bodhi = bodhi_init().await;

    let user = bodhi.request(&UserNameQuery::new("nobody")).await;

    assert!(matches!(user, Err(QueryError::NotFound)));
}
