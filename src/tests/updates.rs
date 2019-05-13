use crate::{BodhiService, UpdateQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_updates() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    UpdateQuery::new().query(&bodhi).unwrap();
}
