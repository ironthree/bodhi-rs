use super::bodhi_init;

use crate::CSRFQuery;

#[tokio::test]
async fn deserialize() {
    let bodhi = bodhi_init().await;

    // query and deserialize a new CSRF token
    bodhi.request(&CSRFQuery::new()).await.unwrap();
}
