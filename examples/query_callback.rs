use std::io::Write;

use bodhi::*;

fn main() -> Result<(), String> {
    let bodhi = BodhiServiceBuilder::default()
        .build()
        .expect("Failed to initialize bodhi client.");

    let mut progress = 0f64;

    let query = BuildQuery::new().releases(FedoraRelease::F31).callback(|p, ps| {
        progress = (p as f64) / (ps as f64);
        print!("\rProgress: {:02}%", (progress * 100f64) as i32);
        std::io::stdout().flush().expect("Failed to flush stdout.");
    });

    bodhi.query(query).expect("Error while querying.");
    println!();

    Ok(())
}
