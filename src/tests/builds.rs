use super::bodhi_init;

use crate::{Build, BuildNVRQuery, BuildQuery, FedoraRelease};

#[tokio::test]
async fn query_sanity_packages() {
    let bodhi = bodhi_init().await;

    let rs_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().packages(vec!["rust"]))
        .await
        .unwrap();
    let go_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().packages(vec!["golang"]))
        .await
        .unwrap();

    let both_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().packages(vec!["rust", "golang"]))
        .await
        .unwrap();

    assert_eq!(both_builds.len(), rs_builds.len() + go_builds.len())
}

#[tokio::test]
async fn query_sanity_releases() {
    let bodhi = bodhi_init().await;

    let f31c_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().releases(vec![FedoraRelease::F31C]))
        .await
        .unwrap();
    let f30c_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().releases(vec![FedoraRelease::F30C]))
        .await
        .unwrap();

    let both_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().releases(vec![FedoraRelease::F31C, FedoraRelease::F30C]))
        .await
        .unwrap();

    assert_eq!(both_builds.len(), f31c_builds.len() + f30c_builds.len())
}

#[tokio::test]
async fn query_sanity_updates() {
    let bodhi = bodhi_init().await;

    let builds_one: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().updates(vec!["FEDORA-2019-cf87377f5f"]))
        .await
        .unwrap();
    let builds_two: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().updates(vec!["FEDORA-2019-24c9d17287"]))
        .await
        .unwrap();

    let both_builds: Vec<Build> = bodhi
        .paginated_request(&BuildQuery::new().updates(vec!["FEDORA-2019-cf87377f5f", "FEDORA-2019-24c9d17287"]))
        .await
        .unwrap();

    assert_eq!(both_builds.len(), builds_one.len() + builds_two.len())
}

#[tokio::test]
async fn nvr_query_some() {
    let bodhi = bodhi_init().await;

    let build = bodhi.request(&BuildNVRQuery::new("rust-1.34.2-1.fc30")).await;

    assert!(build.is_ok());
}

#[tokio::test]
async fn nvr_query_none() {
    let bodhi = bodhi_init().await;

    let build = bodhi.request(&BuildNVRQuery::new("this-doesnt-exist-1-1.fc30")).await;

    assert!(build.is_err());
}
