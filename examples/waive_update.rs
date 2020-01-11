use std::io::{stdin, stdout, Write};
use std::time::Duration;

use bodhi::{BodhiServiceBuilder, Update, UpdateIDQuery};

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

    let bodhi = BodhiServiceBuilder::staging()
        .authentication(&username, &password)
        .timeout(Duration::from_secs(60))
        .build()
        .unwrap();

    let update: Update = match bodhi.query(UpdateIDQuery::new("FEDORA-2019-586c873435")) {
        Err(_) => {
            return Err(String::from("Failed to fetch update."));
        },
        Ok(value) => match value {
            Some(update) => update,
            None => {
                return Err(String::from("Failed to fetch update."));
            },
        },
    };

    let update_waiver = update.waive("Ignore test results.");

    let response = bodhi.edit(&update_waiver);

    match response {
        Ok(edited_update) => {
            println!("{:#?}", edited_update);
            Ok(())
        },
        Err(error) => Err(format!("{:#?}", error)),
    }
}
