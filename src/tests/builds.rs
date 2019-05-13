use crate::{BodhiService, BuildQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_builds() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    BuildQuery::new().query(&bodhi).unwrap();
}
