use bodhi::{BodhiClientBuilder, CSRFQuery};

#[tokio::main]
async fn main() -> Result<(), String> {
    // construct bodhi client for the production instance
    let bodhi = BodhiClientBuilder::default().build().await.unwrap();

    let query = CSRFQuery::new();

    let token = bodhi.request(&query).await.map_err(|error| error.to_string())?;

    println!("CSRF token: {}", token);

    Ok(())
}
