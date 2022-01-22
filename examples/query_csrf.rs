use bodhi::{BodhiClientBuilder, CSRFQuery};

#[tokio::main]
async fn main() {
    // construct bodhi client for the production instance
    let bodhi = BodhiClientBuilder::default().build().await.unwrap();

    let query = CSRFQuery::new();

    let token = bodhi.request(&query).await.unwrap();

    println!("CSRF token: {}", token);
}
