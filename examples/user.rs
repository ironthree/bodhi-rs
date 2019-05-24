use std::env::args;

use bodhi::*;

fn main() -> Result<(), String> {
    let bodhi = BodhiService::new(String::from(FEDORA_BODHI_URL));

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