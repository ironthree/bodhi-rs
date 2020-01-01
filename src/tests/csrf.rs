use super::bodhi_init;

use crate::CSRFQuery;

#[test]
fn deserialize() {
    let bodhi = bodhi_init();

    // query and deserialize a new CSRF token
    bodhi.query(&CSRFQuery::new()).unwrap();
}
