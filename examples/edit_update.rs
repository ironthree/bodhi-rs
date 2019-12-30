use std::io::{stdin, stdout, Write};
use std::time::Duration;

use bodhi::{BodhiServiceBuilder, Update, UpdateEditor, UpdateIDQuery};

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

    let update: Update = match bodhi.query(&UpdateIDQuery::new("FEDORA-2019-586c873435")) {
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

    let notes = format!("{}\n\n...", &update.notes);
    let update_editor = UpdateEditor::from_update(&update).notes(&notes);

    let response = bodhi.edit(&update_editor);

    match response {
        Ok(edited_update) => {
            println!("{:#?}", edited_update);
            Ok(())
        },
        Err(error) => Err(format!("{:#?}", error)),
    }
}
