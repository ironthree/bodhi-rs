use super::bodhi_init;

use crate::error::QueryError;
use crate::{BodhiDate, FedoraRelease, Update, UpdateIDQuery, UpdateQuery};

fn days_ago(x: i64) -> BodhiDate {
    BodhiDate::from(chrono::Utc::now() - chrono::Duration::days(x))
}

#[tokio::test]
async fn query_current() {
    let bodhi = bodhi_init().await;

    let _: Vec<Update> = bodhi
        .paginated_request(
            &UpdateQuery::new()
                .releases(&[&FedoraRelease::CURRENT])
                .submitted_since(&days_ago(2)),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn query_pending() {
    let bodhi = bodhi_init().await;

    let _: Vec<Update> = bodhi
        .paginated_request(
            &UpdateQuery::new()
                .releases(&[&FedoraRelease::PENDING])
                .submitted_since(&days_ago(1)),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn query_archived() {
    let bodhi = bodhi_init().await;

    let _: Vec<Update> = bodhi
        .paginated_request(
            &UpdateQuery::new()
                .releases(&[&FedoraRelease::ARCHIVED])
                .submitted_since(&days_ago(30)),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn query_sanity_aliases() {
    let bodhi = bodhi_init().await;

    let updates_one: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().aliases(&["FEDORA-2019-cf87377f5f"]))
        .await
        .unwrap();
    let updates_two: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().aliases(&["FEDORA-2019-24c9d17287"]))
        .await
        .unwrap();

    let updates_both: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().aliases(&["FEDORA-2019-cf87377f5f", "FEDORA-2019-24c9d17287"]))
        .await
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[tokio::test]
async fn query_sanity_bugs() {
    let bodhi = bodhi_init().await;

    let updates_one: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().bugs(&[1783602]))
        .await
        .unwrap();
    let updates_two: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().bugs(&[1782383]))
        .await
        .unwrap();

    let updates_both: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().bugs(&[1783602, 1782383]))
        .await
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[tokio::test]
async fn query_sanity_builds() {
    let bodhi = bodhi_init().await;

    let updates_one: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().builds(&["rust-1.39.0-1.fc31"]))
        .await
        .unwrap();
    let updates_two: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().builds(&["rust-1.40.0-1.fc31"]))
        .await
        .unwrap();

    let updates_both: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().builds(&["rust-1.39.0-1.fc31", "rust-1.40.0-1.fc31"]))
        .await
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[tokio::test]
async fn query_sanity_packages() {
    let bodhi = bodhi_init().await;

    let updates_one: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().packages(&["granite"]))
        .await
        .unwrap();
    let updates_two: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().packages(&["python-tinydb"]))
        .await
        .unwrap();

    let updates_both: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().packages(&["granite", "python-tinydb"]))
        .await
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[tokio::test]
async fn query_sanity_releases() {
    let bodhi = bodhi_init().await;

    let f32c = FedoraRelease::try_from("F32C").unwrap();
    let f31c = FedoraRelease::try_from("F31C").unwrap();

    let updates_one: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().releases(&[&f32c]))
        .await
        .unwrap();
    let updates_two: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().releases(&[&f31c]))
        .await
        .unwrap();

    let updates_both: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().releases(&[&f32c, &f31c]))
        .await
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[tokio::test]
async fn query_sanity_users() {
    let bodhi = bodhi_init().await;

    let updates_one: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().users(&["astra"]))
        .await
        .unwrap();
    let updates_two: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().users(&["cipherboy"]))
        .await
        .unwrap();

    let updates_both: Vec<Update> = bodhi
        .paginated_request(&UpdateQuery::new().users(&["astra", "cipherboy"]))
        .await
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[tokio::test]
async fn id_query_ok() {
    let bodhi = bodhi_init().await;

    let update = bodhi.request(&UpdateIDQuery::new("FEDORA-2019-227c137c3f")).await;

    assert!(update.is_ok());
}

#[tokio::test]
async fn id_query_err() {
    let bodhi = bodhi_init().await;

    let update = bodhi.request(&UpdateIDQuery::new("NOPE")).await;

    assert!(matches!(update, Err(QueryError::NotFound)));
}
