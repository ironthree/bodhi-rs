use std::io::{stdin, stdout, Write};

use bodhi::create::OverrideBuilder;
use bodhi::service::BodhiServiceBuilder;

fn main() -> Result<(), String> {
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

    let mut bodhi = BodhiServiceBuilder::staging().build().unwrap();

    let auth = bodhi.authenticate(username, password);

    if let Err(error) = auth {
        return Err(format!("Failed to authenticate: {:?}", error));
    }

    let new_override = OverrideBuilder::new(
        String::from("libcloudproviders-0.3.0-1.fc30"),
        String::from("Test buildroot override created by bodhi-rs."),
        String::from("2019-08-31"),
    );

    let response = new_override.create(&bodhi);

    match response {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("{:?}", error)),
    }
}
