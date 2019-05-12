/// This is just a small test program that won't be part of any official releases.
use bodhi::{
    BodhiService, BuildNVRQuery, BuildQuery, CSRFQuery, CommentIDQuery, CommentQuery,
    OverrideNVRQuery, OverrideQuery, PackageQuery, ReleaseNameQuery, ReleaseQuery, StackNameQuery,
    StackQuery, UpdateIDQuery, UpdateQuery, UserNameQuery, UserQuery,
};

// TODO: make this configurable
const SERVER_URL: &str = "https://bodhi.fedoraproject.org";

fn main() -> Result<(), String> {
    let bodhi = BodhiService::new(String::from(SERVER_URL));

    let build = BuildNVRQuery::new(String::from("rust-1.34.1-1.fc29")).query(&bodhi);

    match build {
        Ok(build) => println!("Build: {:#?}", build),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let builds = BuildQuery::new()
        .nvr(String::from("rust-1.34.1-1.fc29"))
        .query(&bodhi);

    match builds {
        Ok(builds) => println!("Builds: {:#?}", builds),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let builds = BuildQuery::new()
        .packages(String::from("rust"))
        .releases(String::from("F29"))
        .query(&bodhi);

    match builds {
        Ok(builds) => println!("Builds: {:#?}", builds),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let comment = CommentIDQuery::new(19999).query(&bodhi);

    match comment {
        Ok(comment) => println!("Comment: {:#?}", comment),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let comments = CommentQuery::new()
        .users(String::from("decathorpe"))
        .packages(String::from("kernel"))
        .query(&bodhi);

    match comments {
        Ok(comment) => println!("Comment: {:#?}", comment),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let csrf = CSRFQuery::new().query(&bodhi);

    match csrf {
        Ok(csrf) => println!("CSRF: {}", csrf),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let r#override = OverrideNVRQuery::new(String::from("wingpanel-2.2.1-1.fc28")).query(&bodhi);

    match r#override {
        Ok(r#override) => println!("Override: {:#?}", r#override),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let overrides = OverrideQuery::new()
        .users(String::from("decathorpe"))
        .query(&bodhi);

    match overrides {
        Ok(overrides) => println!("Overrides: {:#?}", overrides),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let packages = PackageQuery::new().name(String::from("rust")).query(&bodhi);

    match packages {
        Ok(packages) => println!("Packages: {:#?}", packages),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let release = ReleaseNameQuery::new(String::from("F30")).query(&bodhi);

    match release {
        Ok(release) => println!("Release: {:#?}", release),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let releases = ReleaseQuery::new().exclude_archived(true).query(&bodhi);

    match releases {
        Ok(releases) => println!("Releases: {:#?}", releases),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let stack = StackNameQuery::new(String::from("SomeStack")).query(&bodhi);

    match stack {
        Ok(stack) => println!("Stack: {:#?}", stack),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let stacks = StackQuery::new().query(&bodhi);

    match stacks {
        Ok(stacks) => println!("Stacks: {:#?}", stacks),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let update = UpdateIDQuery::new(String::from("FEDORA-2019-3dd0cf468e")).query(&bodhi);

    match update {
        Ok(update) => println!("Update: {:#?}", update),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let updates = UpdateQuery::new()
        .users(String::from("decathorpe"))
        .releases(String::from("F30"))
        .query(&bodhi);

    match updates {
        Ok(updates) => println!("Updates: {:#?}", updates),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let user = UserNameQuery::new(String::from("decathorpe")).query(&bodhi);

    match user {
        Ok(user) => println!("User: {:#?}", user),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    let users = UserQuery::new()
        .groups(String::from("provenpackager"))
        .query(&bodhi);

    match users {
        Ok(users) => println!("Users: {:#?}", users),
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(error);
        }
    }

    Ok(())
}
