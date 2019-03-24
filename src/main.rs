#![feature(bind_by_move_pattern_guards)]

extern crate reqwest;
extern crate serde_derive;

use serde_derive::Deserialize;


const SERVER_URL: &str = "https://bodhi.fedoraproject.org";


#[derive(Deserialize, Debug)]
struct UpdateContent {
    update: Update,
    can_edit: bool,
}


#[derive(Deserialize, Debug)]
struct Update {
    title: String,
    autokarma: bool,
    stable_karma: i32,
    unstable_karma: i32,
    requirements: String,
    require_bugs: bool,
    require_testcases: bool,
    display_name: String,
    notes: String,
    r#type: String,
    status: String,
    request: Option<String>,
    severity: String,
    suggest: String,
    locked: bool,
    pushed: bool,
    critpath: bool,
    close_bugs: bool,
    date_submitted: String,
    date_modified: Option<String>,
    date_approved: Option<String>,
    date_pushed: Option<String>,
    date_testing: Option<String>,
    date_stable: Option<String>,
    alias: String,
    old_updateid: Option<String>,
    test_gating_status: String,
    greenwave_summary_string: String,
    greenwave_unsatisfied_requirements: Option<String>,
    meets_testing_requirements: bool,
    url: String,
    release: Release,
    comments: Vec<Comment>,
    builds: Vec<Build>,
    compose: Option<Compose>,
    bugs: Vec<Bug>,
    user: User,
    updateid: String,
    submitter: String,
    karma: i32,
    content_type: String,
    test_cases: Vec<String>,
}


#[derive(Deserialize, Debug)]
struct Release {
    name: String,
    long_name: String,
    version: String,
    id_prefix: String,
    branch: String,
    dist_tag: String,
    stable_tag: String,
    testing_tag: String,
    candidate_tag: String,
    pending_signing_tag: String,
    pending_testing_tag: String,
    pending_stable_tag: String,
    override_tag: String,
    mail_template: String,
    state: String,
    composed_by_bodhi: bool,
    composes: Vec<Compose>,
}


#[derive(Deserialize, Debug)]
struct Comment {
    id: i32,
    karma: i32,
    karma_critpath: i32,
    text: String,
    anonymous: bool,
    timestamp: String,
    update_id: i32,
    user_id: i32,
    // bug_feedback: Vec<Feedback>, ???
    // testcase_feedback: Vec<TestcaseFeedback>, ???
    user: User,
}


#[derive(Deserialize, Debug)]
struct Compose {}


#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
    email: Option<String>,
    show_popups: bool,
    avatar: String,
    openid: String,
    groups: Vec<Group>,
}


#[derive(Deserialize, Debug)]
struct Build {
    nvr: String,
    release_id: i32,
    signed: bool,
    // ci_url: Option<String>, ???
    r#type: String,
    epoch: i32,
}


#[derive(Deserialize, Debug)]
struct Bug {
    bug_id: i32,
    title: String,
    security: bool,
    parent: bool,
    // feedback: Vec<String>, ???
}


#[derive(Deserialize, Debug)]
struct Group {
    name: String,
}


#[derive(Deserialize, Debug)]
struct BodhiError {
    status: String,
    errors: Vec<ErrorDetails>,
}


#[derive(Deserialize, Debug)]
struct ErrorDetails {
    location: String,
    name: String,
    description: String,
}


struct BodhiService {
    url: String,
}


impl BodhiService {
    fn new(url: String) -> BodhiService {
        BodhiService { url }
    }

    fn request(&self, path: &String) -> Result<reqwest::Response, String> {
        let request_url = format!("{}{}", self.url, path);
        match reqwest::get(&request_url) {
            Ok(response) => Ok(response),
            Err(error) => Err(format!("{:?}", error))
        }
    }

    fn get_build(&self, nvr: &String) -> Result<Option<Build>, String> {
        let path = format!("/builds/{}", nvr);
        let mut response = self.request(&path)?;

        if response.status() == 404 {
            let error: BodhiError = match response.json() {
                Ok(error) => error,
                Err(error) => { return Err(format!("Unexpected response: {:?}", error)); }
            };

            match error.errors.get(0) {
                Some(error) => {
                    if error.description == "No such build" {
                        return Ok(None);
                    }
                },
                None => { return Err(String::from("Unexpected response")); }
            }
        }

        let build: Build = match response.json() {
            Ok(value) => value,
            Err(error) => { return Err(format!("{:?}", error)); }
        };

        Ok(Some(build))
    }
}


fn main() {
    let request_url = format!("{}/updates/FEDORA-2019-062a95fb09", SERVER_URL);
    let mut response = match reqwest::get(&request_url) {
        Ok(response) => response,
        Err(error) => {
            println!("Response: {:#?}", error);
            return;
        }
    };

    let update: UpdateContent = match response.json() {
        Ok(value) => value,
        Err(error) => {
            println!("Update: {:#?}", error);
            return;
        }
    };

    println!("Update:");
    println!("{:#?}", update);

    let bodhi = BodhiService::new(String::from("https://bodhi.fedoraproject.org"));

    let nvr = String::from("rubygem-jekyll-watch-2.2.1-1.fc28");

    let build = bodhi.get_build(&nvr);

    match build {
        Ok(build) => println!("Build: {:#?}", build),
        Err(error) => println!("Error: {:#?}", error),
    }
}
