use std::time::Duration;

use crate::{BodhiService, StackQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_stacks() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(Duration::from_secs(120));

    StackQuery::new().query(&bodhi).unwrap();
}
