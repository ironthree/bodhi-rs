use std::io::{stdin, stdout, Write};
use std::time::Duration;

use bodhi::{BodhiServiceBuilder, Update, UpdateEditor, UpdateIDQuery};

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
    let password = rpassword::prompt_password_stdout("FAS password: ").unwrap();

    let bodhi = BodhiServiceBuilder::staging()
        .authentication(&username, &password)
        .timeout(Duration::from_secs(60))
        .build()
        .await
        .unwrap();

    let update: Update = bodhi
        .request(&UpdateIDQuery::new("FEDORA-2019-586c873435"))
        .await
        .map_err(|error| error.to_string())?;

    let notes = format!("{}\n\n...", &update.notes);
    let update_editor = UpdateEditor::from_update(&update).notes(&notes);

    let response = bodhi.request(&update_editor).await;

    match response {
        Ok(edited_update) => {
            println!("{:#?}", edited_update);
            Ok(())
        },
        Err(error) => Err(format!("{:#?}", error)),
    }
}
