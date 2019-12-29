use std::convert::TryFrom;
use std::io::{stdin, stdout, Write};
use std::time::Duration;

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

    // TODO: looks like the staging instance can't create buildroot overrides
    let bodhi = BodhiServiceBuilder::staging()
        .authentication(&username, &password)
        .timeout(Duration::from_secs(300))
        .build()
        .unwrap();

    let expiration_date = BodhiDate::try_from("2019-12-31").unwrap();

    let new_override = OverrideBuilder::new(
        "libcloudproviders-0.3.0-1.fc30",
        "Test buildroot override created by bodhi-rs.",
        &expiration_date,
    );

    let response = bodhi.create(&new_override);

    match response {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("{:?}", error)),
    }
}
