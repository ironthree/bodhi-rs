use std::io::{stdin, stdout, Write};

use bodhi::{BodhiClientBuilder, Karma, NewComment, Update, UpdateIDQuery};

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
    let password = rpassword::prompt_password("FAS password: ").unwrap();

    // construct bodhi client for the staging instance, and
    // authenticate with the supplied username and password
    let bodhi = BodhiClientBuilder::staging()
        .authentication(&username, &password)
        .build()
        .await
        .unwrap();

    let update: Update = bodhi
        .request(&UpdateIDQuery::new("FEDORA-2019-e7f463674c"))
        .await
        .map_err(|error| error.to_string())?;

    // build a new comment for an update that's still in "testing" state,
    // and add some boilerplate text and a karma value
    let new_comment = update
        .comment()
        .text("Test comment from bodhi-rs.")
        .karma(Karma::Positive);

    // create the update on the service
    let response = bodhi.request(&new_comment).await;

    // check the response whether creating the comment was successful
    let new_comment: NewComment = response.map_err(|error| error.to_string())?;

    println!("New comment created:");
    println!("{:#?}", new_comment);

    Ok(())
}
