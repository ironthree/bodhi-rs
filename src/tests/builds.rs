use super::bodhi_init;

use crate::error::QueryError;
use crate::{Build, BuildNVRQuery, BuildQuery, FedoraRelease};

#[tokio::test]
async fn query_sanity_packages() {
    let bodhi = bodhi_init().await;

    let rs_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().packages(&["rust"]))
        .await
        .unwrap();
    let go_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().packages(&["golang"]))
        .await
        .unwrap();

    let both_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().packages(&["rust", "golang"]))
        .await
        .unwrap();

    assert_eq!(both_builds.len(), rs_builds.len() + go_builds.len())
}

#[tokio::test]
async fn query_sanity_releases() {
    let bodhi = bodhi_init().await;

    let f31c = FedoraRelease::try_from("F31C").unwrap();
    let f30c = FedoraRelease::try_from("F30C").unwrap();

    let f31c_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().releases(&[&f31c]))
        .await
        .unwrap();
    let f30c_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().releases(&[&f30c]))
        .await
        .unwrap();

    let both_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().releases(&[&f31c, &f30c]))
        .await
        .unwrap();

    assert_eq!(both_builds.len(), f31c_builds.len() + f30c_builds.len())
}

#[tokio::test]
async fn query_sanity_updates() {
    let bodhi = bodhi_init().await;

    let builds_one: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().updates(&["FEDORA-2019-cf87377f5f"]))
        .await
        .unwrap();
    let builds_two: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().updates(&["FEDORA-2019-24c9d17287"]))
        .await
        .unwrap();

    let both_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().updates(&["FEDORA-2019-cf87377f5f", "FEDORA-2019-24c9d17287"]))
        .await
        .unwrap();

    assert_eq!(both_builds.len(), builds_one.len() + builds_two.len())
}

#[tokio::test]
async fn nvr_query_ok() {
    let bodhi = bodhi_init().await;

    let build = bodhi.request(&BuildNVRQuery::new("rust-1.34.2-1.fc30")).await;

    assert!(build.is_ok());
}

#[tokio::test]
async fn nvr_query_err() {
    let bodhi = bodhi_init().await;

    let build = bodhi.request(&BuildNVRQuery::new("this-doesnt-exist-1-1.fc30")).await;

    assert!(matches!(build, Err(QueryError::NotFound)));
}
