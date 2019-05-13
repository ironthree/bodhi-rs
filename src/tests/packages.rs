use std::time::Duration;

use crate::{BodhiService, PackageQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_packages() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(Duration::from_secs(120));

    PackageQuery::new().query(&bodhi).unwrap();
}
