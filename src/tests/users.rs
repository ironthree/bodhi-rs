use crate::{BodhiService, UserQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_users() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    UserQuery::new().query(&bodhi).unwrap();
}
