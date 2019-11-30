use std::env::args;

use bodhi::error::QueryError;
use bodhi::query::UserNameQuery;
use bodhi::service::BodhiServiceBuilder;

fn main() -> Result<(), QueryError> {
    let bodhi = BodhiServiceBuilder::default().build().unwrap();

    let mut arguments = args();

    // skip 0th argument (self)
    arguments.next();

    for argument in arguments {
        println!("User: {}", argument);

        let user = UserNameQuery::new(argument).query(&bodhi)?;

        println!("{:#?}", user);
    }

    Ok(())
}
