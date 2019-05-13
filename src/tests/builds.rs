use std::time::Duration;

use crate::{BodhiService, BuildQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_builds() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(Duration::from_secs(120));

    BuildQuery::new().query(&bodhi).unwrap();
}
