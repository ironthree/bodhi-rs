use std::io::{stdin, stdout, Write};

use bodhi::{BodhiServiceBuilder, OverrideEditor, OverrideNVRQuery};

fn read_username() -> String {
    print!("FAS username: ");
    stdout().flush().unwrap();

    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();

    username.trim().to_string()
}

fn main() -> Result<(), String> {
    let username = read_username();
    let password = rpassword::prompt_password_stdout("FAS password: ").unwrap();

    // beware: it looks like the staging instance can't create buildroot overrides
    let bodhi = BodhiServiceBuilder::staging()
        .authentication(&username, &password)
        .build()
        .unwrap();

    let over_ride = match bodhi.query(OverrideNVRQuery::new("elementary-theme-5.4.0-1.fc30")) {
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
