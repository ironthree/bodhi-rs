use std::env::args;

use bodhi::{BodhiServiceBuilder, UserNameQuery};

fn main() -> Result<(), String> {
    // construct bodhi client for the production instance
    let bodhi = BodhiServiceBuilder::default().build().unwrap();

    let mut arguments = args();

    // skip 0th argument (program name)
    arguments.next();

    for argument in arguments {
        println!("User: {}", argument);

        let user = match bodhi.query(&UserNameQuery::new(argument)) {
            Ok(user) => user,
            Err(error) => return Err(format!("{}", error)),
        };

        println!("{:#?}", user);
    }

    Ok(())
}
