/// This is just a small test program that won't be part of any official releases.
use bodhi::{BodhiService, BuildNVRQuery, BuildQuery, CommentIDQuery};

// TODO: make this configurable
const SERVER_URL: &str = "https://bodhi.fedoraproject.org";

fn main() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    let build = BuildNVRQuery::new(String::from("rust-1.34.1-1.fc29")).query(&bodhi);

    match build {
        Ok(build) => println!("Build: {:#?}", build),
        Err(error) => println!("Error: {:#?}", error),
    }

    let builds = BuildQuery::new()
        .nvr(String::from("rust-1.34.1-1.fc29"))
        .query(&bodhi);

    match builds {
        Ok(builds) => println!("Builds: {:#?}", builds),
        Err(error) => println!("Error: {:#?}", error),
    }

    let builds = BuildQuery::new()
        .package(String::from("rust"))
        .release(String::from("F29"))
        .query(&bodhi);

    match builds {
        Ok(builds) => println!("Builds: {:#?}", builds),
        Err(error) => println!("Error: {:#?}", error),
    }

    let comment = CommentIDQuery::new(19999).query(&bodhi);

    match comment {
        Ok(comment) => println!("Comment: {:#?}", comment),
        Err(error) => println!("Error: {:#?}", error),
    }

    /*
    let comments = CommentQuery::new()
        .query(&bodhi);

    match comments {
        Ok(comment) => println!("Comment: {:#?}", comment),
        Err(error) => println!("Error: {:#?}", error),
    }
    */
}
