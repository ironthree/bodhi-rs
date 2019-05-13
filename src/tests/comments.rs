use crate::{BodhiService, CommentQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_comments() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    CommentQuery::new().query(&bodhi).unwrap();
}
