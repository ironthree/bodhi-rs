use crate::{BodhiService, OverrideQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_overrides() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    OverrideQuery::new().query(&bodhi).unwrap();
}
