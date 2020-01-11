use std::io::Write;

use bodhi::*;

fn main() -> Result<(), String> {
    let bodhi = BodhiServiceBuilder::default()
        .build()
        .expect("Failed to initialize bodhi client.");

    let query = BuildQuery::new().releases(FedoraRelease::F31).callback(|p, ps| {
        print!("\rProgress: {:02}%", ((p as f64) / (ps as f64) * 100f64) as i32);
        std::io::stdout().flush().expect("Failed to flush stdout.");
    });

    bodhi.query(query).expect("Error while querying.");
    println!();

    Ok(())
}
