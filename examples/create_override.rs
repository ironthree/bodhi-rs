use std::convert::TryFrom;
use std::io::{stdin, stdout, Write};

use bodhi::{BodhiDate, BodhiServiceBuilder, OverrideBuilder};

fn main() -> Result<(), String> {
    // get username and password from standard input
    let mut username = String::new();
    let mut password = String::new();

    print!("FAS username: ");
    stdout().flush().unwrap();

    if let Err(error) = stdin().read_line(&mut username) {
        return Err(error.to_string());
    }
    let username = username.trim().to_string();

    print!("FAS password: ");
    stdout().flush().unwrap();

    if let Err(error) = stdin().read_line(&mut password) {
        return Err(error.to_string());
    }
    let password = password.trim().to_string();

    // beware: it looks like the staging instance can't create buildroot overrides
    let bodhi = BodhiServiceBuilder::staging()
        .authentication(&username, &password)
        .build()
        .unwrap();

    let expiration_date = BodhiDate::try_from("2020-01-01").unwrap();

    let new_override = OverrideBuilder::new(
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
