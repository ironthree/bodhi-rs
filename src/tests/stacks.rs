use crate::{BodhiService, StackQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_stacks() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    StackQuery::new().query(&bodhi).unwrap();
}
