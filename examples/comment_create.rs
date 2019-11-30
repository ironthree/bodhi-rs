use std::io::{stdin, stdout, Write};

use bodhi::create::CommentBuilder;
use bodhi::data::*;
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

    let new_comment = CommentBuilder::new(String::from("FEDORA-2019-e39d4910c6"))
        .text(String::from("Test comment from bodhi-rs."))
        .karma(Karma::Positive);

    let response = new_comment.create(&bodhi);

    match response {
        Ok(new_comment) => {
            println!("New comment created:");
            println!("{:#?}", new_comment);
            Ok(())
        }
        Err(error) => Err(format!("{:?}", error)),
    }
}
