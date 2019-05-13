use crate::{BodhiService, ReleaseQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_releases() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    ReleaseQuery::new().query(&bodhi).unwrap();
}
