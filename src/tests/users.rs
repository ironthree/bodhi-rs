use std::time::Duration;

use crate::{BodhiService, UserQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_users() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(Duration::from_secs(120));

    UserQuery::new().query(&bodhi).unwrap();
}
