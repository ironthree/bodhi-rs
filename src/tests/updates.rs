use std::time::Duration;

use crate::{BodhiService, UpdateQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_updates() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(Duration::from_secs(120));

    UpdateQuery::new().query(&bodhi).unwrap();
}
