use super::bodhi_init;

use crate::{BodhiDate, FedoraRelease, Update, UpdateIDQuery, UpdateQuery};

fn days_ago(x: i64) -> BodhiDate {
    BodhiDate::from(chrono::Utc::now() - chrono::Duration::days(x))
}

#[test]
fn query_current() {
    let bodhi = bodhi_init();

    let _: Vec<Update> = bodhi
        .query(
            UpdateQuery::new()
                .releases(vec![FedoraRelease::Current])
                .submitted_since(&days_ago(2)),
        )
        .unwrap();
}

#[test]
fn query_pending() {
    let bodhi = bodhi_init();

    let _: Vec<Update> = bodhi
        .query(
            UpdateQuery::new()
                .releases(vec![FedoraRelease::Pending])
                .submitted_since(&days_ago(1)),
        )
        .unwrap();
}

#[test]
fn query_archived() {
    let bodhi = bodhi_init();

    let _: Vec<Update> = bodhi
        .query(
            UpdateQuery::new()
                .releases(vec![FedoraRelease::Archived])
                .submitted_since(&days_ago(30)),
        )
        .unwrap();
}

#[test]
fn query_sanity_aliases() {
    let bodhi = bodhi_init();

    let updates_one: Vec<Update> = bodhi
        .query(UpdateQuery::new().aliases(vec!["FEDORA-2019-cf87377f5f"]))
        .unwrap();
    let updates_two: Vec<Update> = bodhi
        .query(UpdateQuery::new().aliases(vec!["FEDORA-2019-24c9d17287"]))
        .unwrap();

    let updates_both: Vec<Update> = bodhi
        .query(UpdateQuery::new().aliases(vec!["FEDORA-2019-cf87377f5f", "FEDORA-2019-24c9d17287"]))
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[test]
fn query_sanity_bugs() {
    let bodhi = bodhi_init();

    let updates_one: Vec<Update> = bodhi.query(UpdateQuery::new().bugs(vec![1783602])).unwrap();
    let updates_two: Vec<Update> = bodhi.query(UpdateQuery::new().bugs(vec![1782383])).unwrap();

    let updates_both: Vec<Update> = bodhi.query(UpdateQuery::new().bugs(vec![1783602, 1782383])).unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[test]
fn query_sanity_builds() {
    let bodhi = bodhi_init();

    let updates_one: Vec<Update> = bodhi
        .query(UpdateQuery::new().builds(vec!["rust-1.39.0-1.fc31"]))
        .unwrap();
    let updates_two: Vec<Update> = bodhi
        .query(UpdateQuery::new().builds(vec!["rust-1.40.0-1.fc31"]))
        .unwrap();

    let updates_both: Vec<Update> = bodhi
        .query(UpdateQuery::new().builds(vec!["rust-1.39.0-1.fc31", "rust-1.40.0-1.fc31"]))
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[test]
fn query_sanity_packages() {
    let bodhi = bodhi_init();

    let updates_one: Vec<Update> = bodhi.query(UpdateQuery::new().packages(vec!["granite"])).unwrap();
    let updates_two: Vec<Update> = bodhi.query(UpdateQuery::new().packages(vec!["python-tinydb"])).unwrap();

    let updates_both: Vec<Update> = bodhi
        .query(UpdateQuery::new().packages(vec!["granite", "python-tinydb"]))
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[test]
fn query_sanity_releases() {
    let bodhi = bodhi_init();

    let updates_one: Vec<Update> = bodhi
        .query(UpdateQuery::new().releases(vec![FedoraRelease::F32C]))
        .unwrap();
    let updates_two: Vec<Update> = bodhi
        .query(UpdateQuery::new().releases(vec![FedoraRelease::F31C]))
        .unwrap();

    let updates_both: Vec<Update> = bodhi
        .query(UpdateQuery::new().releases(vec![FedoraRelease::F32C, FedoraRelease::F31C]))
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[test]
fn query_sanity_users() {
    let bodhi = bodhi_init();

    let updates_one: Vec<Update> = bodhi.query(UpdateQuery::new().users(vec!["astra"])).unwrap();
    let updates_two: Vec<Update> = bodhi.query(UpdateQuery::new().users(vec!["cipherboy"])).unwrap();

    let updates_both: Vec<Update> = bodhi
        .query(UpdateQuery::new().users(vec!["astra", "cipherboy"]))
        .unwrap();

    assert_eq!(updates_both.len(), updates_one.len() + updates_two.len())
}

#[test]
fn id_query_some() {
    let bodhi = bodhi_init();

    let update: Option<Update> = bodhi.query(UpdateIDQuery::new("FEDORA-2019-227c137c3f")).unwrap();

    assert!(update.is_some());
}

#[test]
fn id_query_none() {
    let bodhi = bodhi_init();

    let update: Option<Update> = bodhi.query(UpdateIDQuery::new("NOPE")).unwrap();

    assert!(update.is_none());
}
