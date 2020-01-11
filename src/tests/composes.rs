use super::bodhi_init;

use crate::ComposeQuery;

#[test]
fn deserialize() {
    let bodhi = bodhi_init();

    // query and deserialize currently active composes
    bodhi.query(ComposeQuery::new()).unwrap();
}
