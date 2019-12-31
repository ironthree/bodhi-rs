use std::io::{stdin, stdout, Write};

use bodhi::{BodhiServiceBuilder, OverrideEditor, OverrideNVRQuery};

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
        .build()
        .unwrap();

    let over_ride = match bodhi.query(&OverrideNVRQuery::new("elementary-theme-5.4.0-1.fc30")) {
        Ok(o) => match o {
            Some(o) => o,
            None => {
                return Err(String::from("Buildroot override not found."));
            },
        },
        Err(_) => {
            return Err(String::from("Buildroot override not found."));
        },
    };

    let override_edit = OverrideEditor::from_override(&over_ride).expired(true);

    let response = bodhi.edit(&override_edit);

    match response {
        Ok(value) => Ok(println!("{:#?}", value)),
        Err(error) => Err(format!("{:#?}", error)),
    }
}
