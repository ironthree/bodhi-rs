use std::time::Duration;

use crate::{BodhiService, CommentQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_comments() {
    let bodhi = BodhiService::new(String::from(SERVER_URL))
        .timeout(Duration::from_secs(120));

    CommentQuery::new().query(&bodhi).unwrap();
}
