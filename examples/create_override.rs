use std::convert::TryFrom;
use std::io::{stdin, stdout, Write};

use bodhi::{BodhiDate, BodhiServiceBuilder, OverrideCreator};

fn read_username() -> String {
    print!("FAS username: ");
    stdout().flush().unwrap();

    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();

    username.trim().to_string()
}

fn main() -> Result<(), String> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let username = read_username();
    let password = rpassword::prompt_password_stdout("FAS password: ").unwrap();

    // beware: it looks like the staging instance can't create buildroot overrides
    let bodhi = BodhiServiceBuilder::staging()
        .authentication(&username, &password)
        .build()
        .unwrap();

    let expiration_date = BodhiDate::try_from("2020-01-01").unwrap();

    let new_override = OverrideCreator::new(
        "elementary-theme-5.4.0-1.fc30",
        "Test buildroot override.",
        &expiration_date,
    );

    let response = bodhi.create(&new_override);

    match response {
        Ok(value) => Ok(println!("{:#?}", value)),
        Err(error) => Err(format!("{:#?}", error)),
    }
}
