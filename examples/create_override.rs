use std::convert::TryFrom;
use std::io::{stdin, stdout, Write};

use bodhi::{BodhiClientBuilder, BodhiDate, NewOverride, OverrideCreator};

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

    // beware: it looks like the staging instance can't create buildroot overrides
    let bodhi = BodhiClientBuilder::staging()
        .authentication(&username, &password)
        .build()
        .await
        .unwrap();

    let expiration_date = BodhiDate::try_from("2020-01-01").unwrap();

    let new_override = OverrideCreator::new(
        "elementary-theme-5.4.0-1.fc30",
        "Test buildroot override.",
        &expiration_date,
    );

    let response = bodhi.request(&new_override).await;

    // check the response whether creating the override was successful
    let new_override: NewOverride = response.map_err(|error| error.to_string())?;

    println!("New override created:");
    println!("{new_override:#?}");

    Ok(())
}
