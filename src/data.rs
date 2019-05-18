use std::collections::HashMap;

use serde::Deserialize;

/// base URL of the fedora bodhi instance
pub const FEDORA_BODHI_URL: &str = "https://bodhi.fedoraproject.org";

/// base URL of the fedora bodhi staging instance
pub const FEDORA_BODHI_STAGING_URL: &str = "https://bodhi.stg.fedoraproject.org";

/// This enum represents the content type of a bodhi update.
pub enum ContentType {
    Base,
    Container,
    Flatpak,
    Module,
    RPM,
}

impl Into<String> for ContentType {
    fn into(self) -> String {
        match self {
            ContentType::Base => String::from("base"),
            ContentType::Container => String::from("container"),
            ContentType::Flatpak => String::from("flatpak"),
            ContentType::Module => String::from("module"),
            ContentType::RPM => String::from("rpm"),
        }
    }
}

/// This enum represents a requested state change of an update.
pub enum UpdateRequest {
    Batched,
    Obsolete,
    Revoke,
    Stable,
    Testing,
    Unpush,
}

impl Into<String> for UpdateRequest {
    fn into(self) -> String {
        match self {
            UpdateRequest::Batched => String::from("batched"),
            UpdateRequest::Obsolete => String::from("obsolete"),
            UpdateRequest::Revoke => String::from("revoke"),
            UpdateRequest::Stable => String::from("stable"),
            UpdateRequest::Testing => String::from("testing"),
            UpdateRequest::Unpush => String::from("unpush"),
        }
    }
}

/// This enum represents the associated severity of a bodhi update.
pub enum UpdateSeverity {
    High,
    Low,
    Medium,
    Unspecified,
    Urgent,
}

impl Into<String> for UpdateSeverity {
    fn into(self) -> String {
        match self {
            UpdateSeverity::High => String::from("high"),
            UpdateSeverity::Low => String::from("low"),
            UpdateSeverity::Medium => String::from("medium"),
            UpdateSeverity::Unspecified => String::from("unspecified"),
            UpdateSeverity::Urgent => String::from("urgent"),
        }
    }
}

/// This enum represents the current state of a bodhi update.
pub enum UpdateStatus {
    Obsolete,
    Pending,
    Processing,
    SideTagActive,
    SideTagExpired,
    Stable,
    Testing,
    Unpushed,
}

impl Into<String> for UpdateStatus {
    fn into(self) -> String {
        match self {
            UpdateStatus::Obsolete => String::from("obsolete"),
            UpdateStatus::Pending => String::from("pending"),
            UpdateStatus::Processing => String::from("processing"),
            UpdateStatus::SideTagActive => String::from("side_tag_active"),
            UpdateStatus::SideTagExpired => String::from("side_tag_expired"),
            UpdateStatus::Stable => String::from("stable"),
            UpdateStatus::Testing => String::from("testing"),
            UpdateStatus::Unpushed => String::from("unpushed"),
        }
    }
}

/// This enum represents the associated suggested action for a bodhi update.
pub enum UpdateSuggestion {
    Logout,
    Reboot,
    Unspecified,
}

impl Into<String> for UpdateSuggestion {
    fn into(self) -> String {
        match self {
            UpdateSuggestion::Logout => String::from("logout"),
            UpdateSuggestion::Reboot => String::from("reboot"),
            UpdateSuggestion::Unspecified => String::from("unspecified"),
        }
    }
}

/// This enum represents the type of a bodhi update.
pub enum UpdateType {
    BugFix,
    Enhancement,
    NewPackage,
    Security,
}

impl Into<String> for UpdateType {
    fn into(self) -> String {
        match self {
            UpdateType::BugFix => String::from("bugfix"),
            UpdateType::Enhancement => String::from("security"),
            UpdateType::NewPackage => String::from("newpackage"),
            UpdateType::Security => String::from("enhancement"),
        }
    }
}

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
    #[serde(rename(deserialize = "type"))]
    pub build_type: String,
    pub ci_url: Option<String>,
    pub epoch: Option<i32>,
    pub nvr: String,
    pub release_id: Option<i32>,
    pub signed: bool,
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
    #[serde(rename(deserialize = "type"))]
    pub package_type: String,
    pub requirements: Option<String>,
    pub stack: Option<Stack>,
    pub stack_id: Option<i32>,
}

/// This struct represents a fedora release as present in the bodhi database.
/// This includes variants (Modular, Container, Flatpak), identified with
/// the "C", "F", and "M" suffixes.
#[derive(Debug, Deserialize)]
pub struct Release {
    pub branch: String,
    pub candidate_tag: String,
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
/// bugs, builds, comments, release, status, submitter, etc.
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
    pub karma: Option<i32>,
    pub locked: bool,
    pub meets_testing_requirements: bool,
    pub notes: String,
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
    pub unstable_karma: Option<i32>,
    #[serde(rename(deserialize = "type"))]
    pub update_type: String,
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
