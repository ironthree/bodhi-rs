//! Query bodhi for all users given as arguments on the command line, and print their details.

use std::env::args;

use bodhi::{BodhiServiceBuilder, UserNameQuery};

fn main() -> Result<(), String> {
    // construct bodhi client for the production instance
    let bodhi = BodhiServiceBuilder::default().build().unwrap();

    let mut arguments = args();

    // skip 0th argument (program name)
    arguments.next();

    for argument in arguments {
        let user = match bodhi.query(&UserNameQuery::new(&argument)) {
            Err(error) => return Err(format!("{}", error)),
            Ok(user) => match user {
                Some(user) => user,
                None => {
                    println!("User '{}' not found.", &argument);
                    println!();
                    continue;
                },
            },
        };

        println!("{}", user);
        println!();
    }

    Ok(())
}
