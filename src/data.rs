use std::collections::HashMap;

use serde::Deserialize;

// TODO: release (F30, F30M, etc.) enum
// TODO: update content_type enum
// TODO: update request enum
// TODO: update severity enum
// TODO: update status enum
// TODO: update suggest enum
// TODO: update type enum

/// This struct contains error messages that are deserialized from bodhi's error responses.
/// TODO: make this a proper error
#[derive(Debug, Deserialize)]
pub struct BodhiError {
    pub errors: Vec<HashMap<String, String>>,
    pub status: String,
}

/// This struct represents a specific BugZilla bug that is associated with an update.
#[derive(Debug, Deserialize)]
pub struct Bug {
    pub bug_id: i32,
    pub feedback: Option<Vec<BugFeedback>>,
    pub parent: bool,
    pub security: bool,
    pub title: Option<String>,
}

/// This struct represents an update feedback item associated with a specific bug.
#[derive(Debug, Deserialize)]
pub struct BugFeedback {
    pub bug: Option<Bug>,
    pub bug_id: i32,
    pub comment_id: i32,
    pub karma: i32,
}

/// This struct represents a specific koji build that bodhi is aware of.
/// This does not include "rpm" or "module" builds for rawhide (yet).
#[derive(Debug, Deserialize)]
pub struct Build {
    pub ci_url: Option<String>,
    pub epoch: Option<i32>,
    pub nvr: String,
    pub release_id: Option<i32>,
    pub signed: bool,
    pub r#type: String,
}

/// This struct represents one comment against a specific update,
/// along with its associated bug and test case feedback.
#[derive(Debug, Deserialize)]
pub struct Comment {
    pub anonymous: bool,
    pub author: Option<String>,
    pub bug_feedback: Vec<BugFeedback>,
    pub id: i32,
    pub karma: i32,
    pub karma_critpath: i32,
    pub testcase_feedback: Vec<TestCaseFeedback>,
    pub text: String,
    pub timestamp: String,
    pub update: Option<Update>,
    pub update_id: i32,
    pub update_title: Option<String>,
    pub user: User,
    pub user_id: i32,
}

/// This struct represents a currently running compose process.
#[derive(Debug, Deserialize)]
pub struct Compose {
    pub checkpoints: String,
    pub content_type: String,
    pub date_created: String,
    pub error_message: Option<String>,
    pub release: Option<Release>,
    pub release_id: Option<i32>,
    pub request: String,
    pub security: bool,
    pub state: String,
    pub state_date: String,
    pub update_summary: Vec<HashMap<String, String>>,
}

/// This struct represents a group from the fedora accounts system (FAS).
#[derive(Debug, Deserialize)]
pub struct Group {
    pub name: String,
}

/// This struct represents a buildroot override, along with the associated build.
#[derive(Debug, Deserialize)]
pub struct Override {
    pub build: Build,
    pub build_id: i32,
    pub expiration_date: String,
    pub expired_date: Option<String>,
    pub notes: String,
    pub nvr: String,
    pub submission_date: String,
    pub submitter: User,
    pub submitter_id: i32,
}

/// This struct represents a specific fedora package.
#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub requirements: Option<String>,
    pub stack: Option<Stack>,
    pub stack_id: Option<i32>,
    pub r#type: String,
}

/// This struct represents a fedora release as present in the bodhi database.
/// This includes variants (Modular, Container, Flatpak), identified with
/// the "C", "F", and "M" suffixes.
#[derive(Debug, Deserialize)]
pub struct Release {
    pub branch: String,
    pub candidate_tag: String,
    pub composes: Vec<Compose>,
    pub composed_by_bodhi: bool,
    pub dist_tag: String,
    pub id_prefix: String,
    pub long_name: String,
    pub mail_template: String,
    pub name: String,
    pub override_tag: String,
    pub pending_signing_tag: String,
    pub pending_stable_tag: String,
    pub pending_testing_tag: String,
    pub stable_tag: String,
    pub state: String,
    pub testing_tag: String,
    pub version: String,
}

/// This struct represents a specific stack in bodhi. It doesn't seem to be in use yet.
#[derive(Debug, Deserialize)]
pub struct Stack {
    pub description: String,
    pub groups: Option<Vec<Group>>,
    pub name: String,
    pub packages: Option<Vec<Package>>,
    pub requirements: String,
    pub users: Vec<User>,
}

/// This struct represents a specific test case as associated with
/// a given test case feedback and update.
#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub package: Option<Package>,
    pub package_id: i32,
}

/// This struct represents an update feedback item associated with a specific test case.
#[derive(Debug, Deserialize)]
pub struct TestCaseFeedback {
    pub comment_id: i32,
    pub karma: i32,
    pub testcase: TestCase,
    pub testcase_id: i32,
}

/// This struct represents a bodhi update, with associated items:
/// bugs, builds, comments, running composes, release, status, submitter, etc.
/// FIXME: old_updateid and updateid are either Strings (aliases) or i32s (IDs),
///        depending on the query
#[derive(Debug, Deserialize)]
pub struct Update {
    pub alias: String,
    pub autokarma: bool,
    pub bugs: Vec<Bug>,
    pub builds: Vec<Build>,
    pub close_bugs: bool,
    pub comments: Option<Vec<Comment>>,
    pub compose: Option<Compose>,
    pub content_type: Option<String>,
    pub critpath: bool,
    pub date_approved: Option<String>,
    pub date_modified: Option<String>,
    pub date_pushed: Option<String>,
    pub date_stable: Option<String>,
    pub date_submitted: Option<String>,
    pub date_testing: Option<String>,
    pub display_name: String,
    pub greenwave_summary_string: Option<String>,
    pub greenwave_unsatisfied_requirements: Option<String>,
    pub karma: i32,
    pub locked: bool,
    pub meets_testing_requirements: bool,
    pub notes: String,
    // FIXME: old_updateid: Option<String>, or Option<i32>?
    pub pushed: bool,
    pub release: Release,
    pub request: Option<String>,
    pub require_bugs: bool,
    pub require_testcases: bool,
    pub requirements: Option<String>,
    pub severity: String,
    pub stable_karma: Option<i32>,
    pub status: String,
    pub submitter: Option<String>,
    pub suggest: String,
    pub test_cases: Option<Vec<TestCase>>,
    pub test_gating_status: Option<String>,
    pub title: String,
    pub r#type: String,
    pub unstable_karma: Option<i32>,
    // FIXME updateid: String, or i32?
    pub url: String,
    pub user: User,
}

/// This struct represents a specific fedora user.
#[derive(Debug, Deserialize)]
pub struct User {
    pub avatar: String,
    pub email: Option<String>,
    pub groups: Vec<Group>,
    pub id: i32,
    pub name: String,
    pub openid: String,
    pub show_popups: bool,
}
