use std::io::{stdin, stdout, Write};

use bodhi::{BodhiClientBuilder, EditedOverride, OverrideEditor, OverrideNVRQuery};

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

    // beware: it looks like the staging instance can't create buildroot overrides
    let bodhi = BodhiClientBuilder::staging()
        .authentication(&username, &password)
        .build()
        .await
        .unwrap();

    let over_ride = bodhi
        .request(&OverrideNVRQuery::new("elementary-theme-5.4.0-1.fc30"))
        .await
        .map_err(|error| error.to_string())?;

    let override_edit = OverrideEditor::from_override(&over_ride).expired(true);

    let response = bodhi.request(&override_edit).await;

    // check the response whether editing the override was successful
    let edited_override: EditedOverride = response.map_err(|error| error.to_string())?;

    println!("Override edited:");
    println!("{edited_override:#?}");

    Ok(())
}
