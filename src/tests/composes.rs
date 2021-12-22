use super::bodhi_init;

use crate::ComposeQuery;

#[tokio::test]
async fn deserialize() {
    let bodhi = bodhi_init().await;

    // query and deserialize currently active composes
    bodhi.request(&ComposeQuery::new()).await.unwrap();
}
