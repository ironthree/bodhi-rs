use std::io::{stdin, stdout, Write};
use std::time::Duration;

use bodhi::{BodhiClientBuilder, NewUpdate, UpdateCreator, UpdateType};

fn read_username() -> String {
    print!("FAS username: ");
    stdout().flush().unwrap();

    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();

    username.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), String> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let username = read_username();
    let password = rpassword::prompt_password("FAS password: ").unwrap();

    let bodhi = BodhiClientBuilder::staging()
        .authentication(&username, &password)
        .timeout(Duration::from_secs(60))
        .build()
        .await
        .unwrap();

    let new_update = UpdateCreator::from_builds(
        &["elementary-theme-5.4.0-1.fc30"],
        "Update to version 5.4.0.\n\nRelease notes: https://github.com/elementary/stylesheet/releases/tag/5.4.0",
    )
    .update_type(UpdateType::Enhancement);

    let response = bodhi.request(&new_update).await;

    // check the response whether creating the update was successful
    let new_update: NewUpdate = response.map_err(|error| error.to_string())?;

    println!("New update created:");
    println!("{new_update:#?}");

    Ok(())
}
