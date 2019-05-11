use std::collections::HashMap;

use serde::Deserialize;

// derived from actual bodhi JSON responses,
// verified to successfully deserialize all data returned by the fedora bodhi instance

#[derive(Debug, Deserialize)]
pub struct BodhiError {
    pub errors: Vec<HashMap<String, String>>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct Bug {
    bug_id: i32,
    feedback: Option<Vec<BugFeedback>>,
    parent: bool,
    security: bool,
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BugFeedback {
    bug: Option<Bug>,
    bug_id: i32,
    comment_id: i32,
    karma: i32,
}

#[derive(Debug, Deserialize)]
pub struct Build {
    ci_url: Option<String>,
    epoch: Option<i32>,
    nvr: String,
    release_id: Option<i32>,
    signed: bool,
    r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    anonymous: bool,
    author: String,
    bug_feedback: Vec<BugFeedback>,
    id: i32,
    karma: i32,
    karma_critpath: i32,
    testcase_feedback: Vec<TestCaseFeedback>,
    text: String,
    timestamp: String,
    update: Update,
    update_id: i32,
    update_title: String,
    user: User,
    user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct Compose {
    content_type: String,
    date_created: String,
    error_message: String,
    release: Option<Release>,
    release_id: Option<i32>,
    request: String,
    security: bool,
    state: String,
    state_date: String,
    update_summary: Vec<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct CSRF {
    csrf_token: String,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Markdown {
    html: String,
}

#[derive(Debug, Deserialize)]
pub struct Override {
    build: Build,
    build_id: i32,
    expiration_date: String,
    expired_date: Option<String>,
    notes: String,
    nvr: String,
    submission_date: String,
    submitter: User,
    submitter_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    name: String,
    // requirements: Option<?>,
    // stack: Option<?>,
    // stack_id: Option<i32>,
    r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    branch: String,
    candidate_tag: String,
    composes: Vec<Compose>,
    composed_by_bodhi: bool,
    dist_tag: String,
    id_prefix: String,
    long_name: String,
    mail_template: String,
    name: String,
    override_tag: String,
    pending_signing_tag: String,
    pending_stable_tag: String,
    pending_testing_tag: String,
    stable_tag: String,
    state: String,
    testing_tag: String,
    version: String,
}

#[derive(Debug, Deserialize)]
pub struct Stack {
    description: String,
    // groups: Vec<?>,
    name: String,
    // packages: Vec<?>,
    requirements: String,
    users: Vec<User>,
}

#[derive(Debug, Deserialize)]
pub struct TestCase {
    name: String,
    package: Package,
    package_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseFeedback {
    comment_id: i32,
    karma: i32,
    testcase: TestCase,
    testcase_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct Update {
    alias: String,
    autokarma: bool,
    bugs: Vec<Bug>,
    builds: Vec<Build>,
    close_bugs: bool,
    comments: Option<Vec<Comment>>,
    compose: Option<Compose>,
    content_type: Option<String>,
    critpath: bool,
    date_approved: Option<String>,
    date_modified: Option<String>,
    date_pushed: Option<String>,
    date_stable: Option<String>,
    date_submitted: Option<String>,
    date_testing: Option<String>,
    display_name: String,
    greenwave_summary_string: Option<String>,
    // TODO: greenwave_unsatisfied_requirements: Option<?>,
    karma: i32,
    locked: bool,
    meets_testing_requirements: bool,
    notes: String,
    old_updateid: Option<String>,
    pushed: bool,
    release: Release,
    request: Option<String>,
    require_bugs: bool,
    require_testcases: bool,
    requirements: Option<String>,
    severity: String,
    stable_karma: Option<i32>,
    status: String,
    submitter: Option<String>,
    suggest: String,
    test_cases: Option<Vec<TestCase>>,
    test_gating_status: Option<String>,
    title: String,
    r#type: String,
    unstable_karma: Option<i32>,
    updateid: Option<i32>,
    url: String,
    user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    avatar: String,
    email: Option<String>,
    groups: Vec<Group>,
    id: i32,
    name: String,
    openid: String,
    show_popups: bool,
}

#[derive(Debug, Deserialize)]
struct UserDetails {
    urls: HashMap<String, String>,
    user: User,
}
