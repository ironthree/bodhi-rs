use std::io::Write;

use bodhi::{BodhiServiceBuilder, BuildQuery};

#[tokio::main]
async fn main() {
    // construct bodhi client for the production instance
    let bodhi = BodhiServiceBuilder::default().build().await.unwrap();

    let progressbar = |p, ps| {
        let progress = if ps == 0 {
            0f64
        } else {
            (p as f64) / (ps as f64)
        };

        print!("\rProgress: {:02}%", (progress * 100f64) as i32);

        if p == ps {
            println!();
        }
        std::io::stdout().flush().expect("Failed to flush stdout.");
    };

    let query = BuildQuery::new().updates(vec!["FEDORA-2021-165f1e7af4"]).callback(progressbar);

    let builds = bodhi.paginated_request(&query).await.unwrap();

    println!("Update builds:");
    println!("{:#?}", builds);
}
