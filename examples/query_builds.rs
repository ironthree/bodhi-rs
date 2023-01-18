use std::io::Write;

use bodhi::{BodhiClientBuilder, BuildQuery};

#[tokio::main]
async fn main() -> Result<(), String> {
    // construct bodhi client for the production instance
    let bodhi = BodhiClientBuilder::default().build().await.unwrap();

    let progressbar = |p, ps| {
        let progress = if ps == 0 { 0f64 } else { (p as f64) / (ps as f64) };

        print!("\rProgress: {:02}%", (progress * 100f64) as i32);

        if p == ps {
            println!();
        }
        std::io::stdout().flush().expect("Failed to flush stdout.");
    };

    let query = BuildQuery::new()
        .updates(&["FEDORA-2021-165f1e7af4"])
        .callback(progressbar);

    let builds = bodhi
        .paginated_request(&query)
        .await
        .map_err(|error| error.to_string())?;

    println!("Update builds:");
    println!("{builds:#?}");

    Ok(())
}
