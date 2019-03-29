use std::collections::HashMap;

use serde::Deserialize;


// derived from actual bodhi JSON responses

#[derive(Deserialize, Debug)]
pub struct Build {
    ci_url: Option<String>,
    epoch: i32,
    nvr: String,
    release_id: i32,
    signed: bool,
    r#type: String,
}


// TODO: make private again
#[derive(Deserialize, Debug)]
pub struct BuildListPage {
    builds: Vec<Build>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}


#[derive(Deserialize, Debug)]
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


#[derive(Deserialize, Debug)]
struct BugFeedback {
    bug: Bug,
    bug_id: i32,
    comment_id: i32,
    karma: i32,
}


#[derive(Deserialize, Debug)]
struct Bug {
    bug_id: i32,
    // feedback: Option<Vec<BugFeedback>>, ?? occurs in Update, but not in BugFeedBack (circular dependency?)
    parent: bool,
    security: bool,
    title: String,
}


#[derive(Deserialize, Debug)]
struct TestCaseFeedback {
    comment_id: i32,
    karma: i32,
    testcase: TestCase,
    testcase_id: i32,
}


#[derive(Deserialize, Debug)]
struct TestCase {
    name: String,
    package: Package,
    package_id: i32,
}


#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    // requirements: Option<?>,
    // stack: Option<?>,
    // stack_id: Option<i32>,
    r#type: String,
}


// TODO: make private again
#[derive(Deserialize, Debug)]
pub struct CommentListPage {
    comments: Vec<Comment>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}


#[derive(Deserialize, Debug)]
struct Compose {
    checkpoints: HashMap<String, String>,
    content_type: String,
    date_created: String,
    error_message: String,
    release: Release,
    release_id: i32,
    request: String,
    security: bool,
    state: String,
    state_date: String,
    update_summary: Vec<HashMap<String, String>>,
}


#[derive(Deserialize, Debug)]
struct Release {
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


#[derive(Deserialize, Debug)]
pub struct CSRF {
    csrf_token: String,
}


#[derive(Deserialize, Debug)]
pub struct Markdown {
    html: String,
}


#[derive(Deserialize, Debug)]
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


#[derive(Deserialize, Debug)]
struct OverrideListPage {
    overrides: Vec<Override>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}


// TODO: make private again
#[derive(Deserialize, Debug)]
pub struct PackageListPage {
    packages: Vec<Package>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}


#[derive(Deserialize, Debug)]
struct ReleaseListPage {
    page: i32,
    pages: i32,
    releases: Vec<Release>,
    rows_per_page: i32,
    total: i32,
}


/*
#[derive(Deserialize, Debug)]
struct Stack {
    description: String,
    // groups: Vec<?>,
    name: String,
    // packages: Vec<?>,
    requirements: String,
    users: Vec<User>,
}


#[derive(Deserialize, Debug)]
struct StackListPage {
    page: i32,
    pages: i32,
    rows_per_page: i32,
    stacks: Vec<Stack>,
    total: i32,
}
*/


#[derive(Deserialize, Debug)]
struct Update {
    alias: String,
    autokarma: bool,
    bugs: Vec<Bug>,
    builds: Vec<Build>,
    close_bugs: bool,
    comments: Vec<Comment>,
    compose: Option<Compose>,
    content_type: String,
    critpath: bool,
    date_approved: Option<String>,
    date_modified: Option<String>,
    date_pushed: Option<String>,
    date_stable: Option<String>,
    date_submitted: String,
    date_testing: Option<String>,
    display_name: String,
    greenwave_summary_string: String,
    // greenwave_unsatisfied_requirements: Option<?>,
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
    requirements: String,
    severity: String,
    stable_karma: i32,
    status: String,
    submitter: String,
    suggest: String,
    test_cases: Vec<TestCase>,
    test_gating_status: String,
    title: String,
    r#type: String,
    unstable_karma: i32,
    updateid: String,
    url: String,
    user: User,
}


#[derive(Deserialize, Debug)]
struct User {
    avatar: String,
    email: Option<String>,
    groups: Vec<Group>,
    id: i32,
    name: String,
    openid: String,
    show_popups: bool,
}


#[derive(Deserialize, Debug)]
struct Group {
    name: String,
}


#[derive(Deserialize, Debug)]
struct UserDetails {
    urls: HashMap<String, String>,
    user: User,
}


#[derive(Deserialize, Debug)]
struct UserListPage {
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
    users: Vec<User>,
}
