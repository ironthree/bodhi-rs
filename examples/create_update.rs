use std::io::{stdin, stdout, Write};
use std::time::Duration;

use bodhi::{BodhiServiceBuilder, UpdateBuilder, UpdateType};

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

    let bodhi = BodhiServiceBuilder::staging()
        .authentication(&username, &password)
        .timeout(Duration::from_secs(60))
        .build()
        .unwrap();

    let new_update = UpdateBuilder::from_builds(
        &["elementary-theme-5.4.0-1.fc30"],
        "Update to version 5.4.0.\n\nRelease notes: https://github.com/elementary/stylesheet/releases/tag/5.4.0",
    )
    .update_type(UpdateType::Enhancement);

    let response = bodhi.create(&new_update);

    match response {
        Ok(created_update) => {
            println!("{:#?}", created_update);
            Ok(())
        },
        Err(error) => Err(format!("{:#?}", error)),
    }
}
