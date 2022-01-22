use bodhi::{BodhiClientBuilder, BuildNVRQuery};

#[tokio::main]
async fn main() {
    // construct bodhi client for the production instance
    let bodhi = BodhiClientBuilder::default().build().await.unwrap();

    let query = BuildNVRQuery::new("elementary-theme-5.4.0-1.fc30");

    let build = bodhi.request(&query).await.unwrap();

    println!("Build information:");
    println!("{:?}", build);
}
