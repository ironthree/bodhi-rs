//! Query bodhi for all users given as arguments on the command line, and print their details.

use std::env::args;

use bodhi::error::QueryError;
use bodhi::{BodhiClientBuilder, UserNameQuery};

#[tokio::main]
async fn main() -> Result<(), String> {
    // construct bodhi client for the production instance
    let bodhi = BodhiClientBuilder::default()
        .build()
        .await
        .map_err(|error| error.to_string())?;

    let mut arguments = args();

    // skip 0th argument (program name)
    arguments.next();

    for argument in arguments {
        let user = match bodhi.request(&UserNameQuery::new(&argument)).await {
            Ok(user) => user,
            Err(error) => match error {
                QueryError::NotFound => {
                    println!("User '{}' not found.", &argument);
                    println!();
                    continue;
                },
                error => return Err(error.to_string()),
            },
        };

        println!("{user}");
        println!();
    }

    Ok(())
}
