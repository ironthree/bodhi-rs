use super::bodhi_init;

use crate::{User, UserNameQuery, UserQuery};

#[test]
fn query_sanity_updates() {
    let bodhi = bodhi_init();

    let users_one: Vec<User> = bodhi.query(UserQuery::new().updates("FEDORA-2019-ac2a21ff07")).unwrap();
    let users_two: Vec<User> = bodhi.query(UserQuery::new().updates("FEDORA-2019-ac3dc27f26")).unwrap();

    let users_both: Vec<User> = bodhi
        .query(
            UserQuery::new()
                .updates("FEDORA-2019-ac2a21ff07")
                .updates("FEDORA-2019-ac3dc27f26"),
        )
        .unwrap();

    assert_eq!(users_both.len(), users_one.len() + users_two.len())
}

#[test]
fn name_query_some() {
    let bodhi = bodhi_init();

    let user: Option<User> = bodhi.query(UserNameQuery::new("decathorpe")).unwrap();

    assert!(user.is_some());
}

#[test]
fn name_query_none() {
    let bodhi = bodhi_init();

    let user: Option<User> = bodhi.query(UserNameQuery::new("nobody")).unwrap();

    assert!(user.is_none());
}
