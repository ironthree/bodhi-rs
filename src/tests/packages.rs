use crate::{BodhiService, PackageQuery};
use super::SERVER_URL;

#[test]
fn deserialize_all_packages() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    PackageQuery::new().query(&bodhi).unwrap();
}
