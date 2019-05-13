use std::time::Duration;

use crate::{BodhiService, OverrideQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_overrides() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(Duration::from_secs(120));

    OverrideQuery::new().query(&bodhi).unwrap();
}
