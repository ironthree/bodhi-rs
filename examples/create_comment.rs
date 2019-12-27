use std::io::{stdin, stdout, Write};

use bodhi::{BodhiServiceBuilder, CommentBuilder, Karma};

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

    // construct bodhi client for the staging instance, and
    // authenticate with the supplied username and password
    let bodhi = BodhiServiceBuilder::staging()
        .authentication(username, password)
        .build()
        .unwrap();

    // build a new comment for an update that's still in "testing" state,
    // and add some boilerplate text and a karma value
    let new_comment = CommentBuilder::new(String::from("FEDORA-2019-e7f463674c"))
        .text(String::from("Test comment from bodhi-rs."))
        .karma(Karma::Positive);

    // create the update on the service
    let response = bodhi.create(&new_comment);

    // check the response whether creating the comment was successful
    match response {
        Ok(new_comment) => {
            println!("New comment created:");
            println!("{:#?}", new_comment);
            Ok(())
        },
        Err(error) => Err(format!("{:?}", error)),
    }
}
