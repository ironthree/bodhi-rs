use super::bodhi_init;

use crate::error::QueryError;
use crate::{FedoraRelease, Override, OverrideNVRQuery, OverrideQuery};

#[tokio::test]
async fn query_sanity_packages() {
    let bodhi = bodhi_init().await;

    let rs_overs: Vec<Override> = bodhi
        .paginated_request(&OverrideQuery::new().packages(&["rust"]))
        .await
        .unwrap();
    let go_overs: Vec<Override> = bodhi
        .paginated_request(&OverrideQuery::new().packages(&["golang"]))
        .await
        .unwrap();

    let both_overs: Vec<Override> = bodhi
        .paginated_request(&OverrideQuery::new().packages(&["rust", "golang"]))
        .await
        .unwrap();

    assert_eq!(both_overs.len(), rs_overs.len() + go_overs.len())
}

#[tokio::test]
async fn query_sanity_releases() {
    let bodhi = bodhi_init().await;

    let f31 = || FedoraRelease::try_from("F31").unwrap();
    let f32 = || FedoraRelease::try_from("F32").unwrap();

    let f31_overs: Vec<Override> = bodhi
        .paginated_request(&OverrideQuery::new().releases(&[f31()]))
        .await
        .unwrap();
    let f32_overs: Vec<Override> = bodhi
        .paginated_request(&OverrideQuery::new().releases(&[f32()]))
        .await
        .unwrap();

    let both_overs: Vec<Override> = bodhi
        .paginated_request(&OverrideQuery::new().releases(&[f31(), f32()]))
        .await
        .unwrap();

    assert_eq!(both_overs.len(), f31_overs.len() + f32_overs.len())
}

#[tokio::test]
async fn query_sanity_users() {
    let bodhi = bodhi_init().await;

    let overs_one: Vec<Override> = bodhi
        .paginated_request(&OverrideQuery::new().users(&["gil"]))
        .await
        .unwrap();
    let overs_two: Vec<Override> = bodhi
        .paginated_request(&OverrideQuery::new().users(&["lef"]))
        .await
        .unwrap();

    let both_overs: Vec<Override> = bodhi
        .paginated_request(&OverrideQuery::new().users(&["gil", "lef"]))
        .await
        .unwrap();

    assert_eq!(both_overs.len(), overs_one.len() + overs_two.len())
}

#[tokio::test]
async fn nvr_query_ok() {
    let bodhi = bodhi_init().await;

    let over_ride = bodhi.request(&OverrideNVRQuery::new("rust-1.34.2-1.fc30")).await;

    assert!(over_ride.is_ok());
}

#[tokio::test]
async fn nvr_query_err() {
    let bodhi = bodhi_init().await;

    let over_ride = bodhi.request(&OverrideNVRQuery::new("syncthing-1.1.3-1.fc30")).await;

    assert!(matches!(over_ride, Err(QueryError::NotFound)));
}
