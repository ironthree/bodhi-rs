extern crate bodhi;

use bodhi::BodhiService;

const SERVER_URL: &str = "https://bodhi.fedoraproject.org";


fn main() {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    let nvr = String::from("rubygem-jekyll-watch-2.2.1-1.fc28");

    let build = bodhi.query_build_by_nvr(&nvr);

    match build {
        Ok(build) => println!("Build: {:#?}", build),
        Err(error) => println!("Error: {:#?}", error),
    }

    let builds = bodhi.query_build(None, Some(vec!(String::from("syncthing"))), Some(vec!(String::from("F28"))), None, None, None);

    match builds {
        Ok(build) => println!("Build: {:#?}", build),
        Err(error) => println!("Error: {:#?}", error),
    }
}
