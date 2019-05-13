use std::time::Duration;

use crate::{BodhiService, ReleaseQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_releases() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(Duration::from_secs(120));

    ReleaseQuery::new().query(&bodhi).unwrap();
}
