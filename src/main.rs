extern crate bodhi;

use bodhi::*;

const SERVER_URL: &str = "https://bodhi.fedoraproject.org";


fn main() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    let build = BuildQuery::new()
        .nvr(String::from("rubygem-jekyll-watch-2.2.1-1.fc28"))
        .query(&bodhi);

    match build {
        Ok(build) => println!("Build: {:#?}", build),
        Err(error) => println!("Error: {:#?}", error),
    }

    let builds = BuildQuery::new()
        .query(&bodhi);

    match builds {
        Ok(build) => println!("Build: {:#?}", build),
        Err(error) => println!("Error: {:#?}", error),
    }

    let comment = CommentIDQuery::new(19999)
        .query(&bodhi);

    match comment {
        Ok(Some(comment)) => println!("Comment: {:#?}", comment),
        Ok(None) => println!("Comment: None"),
        Err(error) => println!("Error: {:#?}", error),
    }

    let comments = CommentQuery::new()
        .query(&bodhi);

    match comments {
        Ok(comment) => println!("Comment: {:#?}", comment),
        Err(error) => println!("Error: {:#?}", error),
    }
}
