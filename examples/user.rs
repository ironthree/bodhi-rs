use std::env::args;

use bodhi::{error::QueryError, query::UserNameQuery, service::BodhiServiceBuilder};

fn main() -> Result<(), QueryError> {
    let bodhi = BodhiServiceBuilder::default().build().unwrap();

    let mut arguments = args();

    // skip 0th argument (self)
    arguments.next();

    for argument in arguments {
        println!("User: {}", argument);

        let user = bodhi.query(&UserNameQuery::new(argument))?;

        println!("{:#?}", user);
    }

    Ok(())
}
