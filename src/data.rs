//! This module contains the data types that are needed to deserialize bodhi server responses which
//! are also public outside this crate. Some internal data types for queries are implemented in the
//! corresponding query module.
//!
//! Additionally, the bodhi REST API has some arguments that accept Strings, but only from a limited
//! set of enumerated values. To abstract this, the corresponding query filters accept some of the
//! enum types defined here, instead of the String arguments directly.

use std::convert::TryFrom;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

/// base URL of the fedora bodhi instance
pub const FEDORA_BODHI_URL: &str = "https://bodhi.fedoraproject.org";

/// base URL of the fedora bodhi staging instance
pub const FEDORA_BODHI_STG_URL: &str = "https://bodhi.stg.fedoraproject.org";

/// This enum represents a "Karma" value, which is either a positive (+1), neutral (±0), or negative
/// (-1) feedback for an update, and is associated with a [`Comment`](struct.Comment.html), and
/// possibly also a [`TestCaseFeedback`](struct.TestCase.html) or a
/// [`BugFeedback`](struct.BugFeedback.html).
#[derive(Debug, Clone, Deserialize_repr)]
#[repr(i8)]
pub enum Karma {
    /// positive feedback
    Positive = 1,
    /// neutral / informational feedback
    Neutral = 0,
    /// negative feedback
    Negative = -1,
}

impl Into<i32> for Karma {
    fn into(self) -> i32 {
        match self {
            Karma::Positive => 1,
            Karma::Neutral => 0,
            Karma::Negative => -1,
        }
    }
}

impl Into<String> for Karma {
    fn into(self) -> String {
        match self {
            Karma::Positive => String::from("+1"),
            Karma::Neutral => String::from("0"),
            Karma::Negative => String::from("-1"),
        }
    }
}

impl From<i32> for Karma {
    fn from(karma: i32) -> Karma {
        match karma {
            1 => Karma::Positive,
            0 => Karma::Neutral,
            -1 => Karma::Negative,
            _ => unreachable!(),
        }
    }
}

/// This enum represents a fedora release.
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub enum FedoraRelease {
    F32,
    F32C,
    F31,
    F31C,
    F31F,
    F31M,
    F30,
    F30C,
    F30F,
    F30M,
    F29,
    F29C,
    F29F,
    F29M,
    F28,
    F28C,
    F28M,
    F27,
    F27M,
    F26,
    F25,
    F24,
    F23,
    F22,
    F21,
    #[serde(rename(deserialize = "EPEL-8"))]
    EPEL8,
    #[serde(rename(deserialize = "EPEL-8M"))]
    EPEL8M,
    #[serde(rename(deserialize = "EPEL-7"))]
    EPEL7,
    #[serde(rename(deserialize = "EL-6"))]
    EL6,
    #[serde(rename(deserialize = "EL-5"))]
    EL5,
}

impl TryFrom<&str> for FedoraRelease {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "F32" => Ok(FedoraRelease::F32),
            "F32C" => Ok(FedoraRelease::F32C),
            "F31" => Ok(FedoraRelease::F31),
            "F31C" => Ok(FedoraRelease::F31C),
            "F31F" => Ok(FedoraRelease::F31F),
            "F31M" => Ok(FedoraRelease::F31M),
            "F30" => Ok(FedoraRelease::F30),
            "F30C" => Ok(FedoraRelease::F30C),
            "F30F" => Ok(FedoraRelease::F30F),
            "F30M" => Ok(FedoraRelease::F30M),
            "F29" => Ok(FedoraRelease::F29),
            "F29C" => Ok(FedoraRelease::F29C),
            "F29F" => Ok(FedoraRelease::F29F),
            "F29M" => Ok(FedoraRelease::F29M),
            "F28" => Ok(FedoraRelease::F28),
            "F28C" => Ok(FedoraRelease::F28C),
            "F28M" => Ok(FedoraRelease::F28M),
            "F27" => Ok(FedoraRelease::F27),
            "F27M" => Ok(FedoraRelease::F27M),
            "F26" => Ok(FedoraRelease::F26),
            "F25" => Ok(FedoraRelease::F25),
            "F24" => Ok(FedoraRelease::F24),
            "F23" => Ok(FedoraRelease::F23),
            "F22" => Ok(FedoraRelease::F22),
            "F21" => Ok(FedoraRelease::F21),
            "EPEL-8" => Ok(FedoraRelease::EPEL8),
            "EPEL-8M" => Ok(FedoraRelease::EPEL8M),
            "EPEL-7" => Ok(FedoraRelease::EPEL7),
            "EL-6" => Ok(FedoraRelease::EL6),
            "EL-5" => Ok(FedoraRelease::EL5),
            _ => Err(format!("Unrecognised release: {}", value)),
        }
    }
}

impl Into<String> for FedoraRelease {
    fn into(self) -> String {
        match self {
            FedoraRelease::F32 => String::from("F32"),
            FedoraRelease::F32C => String::from("F32C"),
            FedoraRelease::F31 => String::from("F31"),
            FedoraRelease::F31C => String::from("F31C"),
            FedoraRelease::F31F => String::from("F31F"),
            FedoraRelease::F31M => String::from("F31M"),
            FedoraRelease::F30 => String::from("F30"),
            FedoraRelease::F30C => String::from("F30C"),
            FedoraRelease::F30F => String::from("F30F"),
            FedoraRelease::F30M => String::from("F30M"),
            FedoraRelease::F29 => String::from("F29"),
            FedoraRelease::F29C => String::from("F29C"),
            FedoraRelease::F29F => String::from("F29F"),
            FedoraRelease::F29M => String::from("F29M"),
            FedoraRelease::F28 => String::from("F28"),
            FedoraRelease::F28C => String::from("F28C"),
            FedoraRelease::F28M => String::from("F28M"),
            FedoraRelease::F27 => String::from("F27"),
            FedoraRelease::F27M => String::from("F27M"),
            FedoraRelease::F26 => String::from("F26"),
            FedoraRelease::F25 => String::from("F25"),
            FedoraRelease::F24 => String::from("F24"),
            FedoraRelease::F23 => String::from("F23"),
            FedoraRelease::F22 => String::from("F22"),
            FedoraRelease::F21 => String::from("F21"),
            FedoraRelease::EPEL8 => String::from("EPEL-8"),
            FedoraRelease::EPEL8M => String::from("EPEL-8M"),
            FedoraRelease::EPEL7 => String::from("EPEL-7"),
            FedoraRelease::EL6 => String::from("EL-6"),
            FedoraRelease::EL5 => String::from("EL-5"),
        }
    }
}

/// This enum represents the content type of a bodhi update.
#[derive(Debug, Deserialize)]
pub enum ContentType {
    /// "base" content type (seems to be unused)
    #[serde(rename(deserialize = "base"))]
    Base,
    /// tag for container image updates
    #[serde(rename(deserialize = "container"))]
    Container,
    /// tag for flatpak updates
    #[serde(rename(deserialize = "flatpak"))]
    Flatpak,
    /// tag for module updates
    #[serde(rename(deserialize = "module"))]
    Module,
    /// tag for traditional RPM package updates
    #[serde(rename(deserialize = "rpm"))]
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
#[derive(Debug, Deserialize)]
pub enum UpdateRequest {
    /// request for an update to get "batched" for the next stable push (no longer used)
    #[serde(rename(deserialize = "batched"))]
    Batched,
    /// request for the update to be removed as "obsolete" (usually when another update supersedes
    /// it)
    #[serde(rename(deserialize = "obsolete"))]
    Obsolete,
    /// request for the update to be "revoked" or removed
    #[serde(rename(deserialize = "revoke"))]
    Revoke,
    /// request for the update to get pushed to stable
    #[serde(rename(deserialize = "stable"))]
    Stable,
    /// request for the update to get pushed to testing
    #[serde(rename(deserialize = "testing"))]
    Testing,
    /// request for the update to get "unpushed" (removed) from testing
    #[serde(rename(deserialize = "unpush"))]
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
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub enum UpdateSeverity {
    #[serde(rename(deserialize = "high"))]
    High,
    #[serde(rename(deserialize = "low"))]
    Low,
    #[serde(rename(deserialize = "medium"))]
    Medium,
    #[serde(rename(deserialize = "unspecified"))]
    Unspecified,
    #[serde(rename(deserialize = "urgent"))]
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
#[derive(Debug, Deserialize)]
pub enum UpdateStatus {
    /// tag for updates that have been obsoleted by another update
    #[serde(rename(deserialize = "obsolete"))]
    Obsolete,
    /// tag for updates that are pending for either testing or stable
    #[serde(rename(deserialize = "pending"))]
    Pending,
    /// tag for updates that are still being processed
    #[serde(rename(deserialize = "processing"))]
    Processing,
    /// tag for updates that are associated with an active side tag
    #[serde(rename(deserialize = "side_tag_active"))]
    SideTagActive,
    /// tag for updates that are associated with an expired side tag
    #[serde(rename(deserialize = "side_tag_expired"))]
    SideTagExpired,
    /// tag for updates that have been pushed to stable
    #[serde(rename(deserialize = "stable"))]
    Stable,
    /// tag for updates that have been pushed to testing
    #[serde(rename(deserialize = "testing"))]
    Testing,
    /// tag for updates that have been "unpushed" from testing
    #[serde(rename(deserialize = "unpushed"))]
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
#[derive(Debug, Deserialize)]
pub enum UpdateSuggestion {
    /// recommendation to log out for the update to get applied
    #[serde(rename(deserialize = "logout"))]
    Logout,
    /// recommendation to reboot for the update to get applied
    #[serde(rename(deserialize = "reboot"))]
    Reboot,
    /// no recommendation
    #[serde(rename(deserialize = "unspecified"))]
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
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub enum UpdateType {
    #[serde(rename(deserialize = "bugfix"))]
    BugFix,
    #[serde(rename(deserialize = "security"))]
    Enhancement,
    #[serde(rename(deserialize = "newpackage"))]
    NewPackage,
    #[serde(rename(deserialize = "enhancement"))]
    Security,
    #[serde(rename(deserialize = "unspecified"))]
    Unspecified,
}

impl Into<String> for UpdateType {
    fn into(self) -> String {
        match self {
            UpdateType::BugFix => String::from("bugfix"),
            UpdateType::Enhancement => String::from("enhancement"),
            UpdateType::NewPackage => String::from("newpackage"),
            UpdateType::Security => String::from("security"),
            UpdateType::Unspecified => String::from("unspecified"),
        }
    }
}

/// This struct represents a specific BugZilla bug that is associated with an update.
#[derive(Debug, Deserialize)]
pub struct Bug {
    /// bug ID in the BugZilla system: <https://bugzilla.redhat.com/show_bug.cgi?id={bug_id}>
    pub bug_id: u32,
    /// list of [`BugFeedback`](struct.BugFeedback.html) items associated with this bug
    pub feedback: Option<Vec<BugFeedback>>,
    // what is this?
    parent: bool,
    // what is this?
    security: bool,
    /// title of the bug in BugZilla
    pub title: Option<String>,
}

/// This struct represents an update feedback item associated with a specific bug.
#[derive(Debug, Deserialize)]
pub struct BugFeedback {
    pub bug: Option<Bug>,
    pub bug_id: u32,
    pub comment_id: Option<u32>,
    pub karma: Karma,
}

/// This struct represents a specific koji build that bodhi is aware of.
#[derive(Debug, Deserialize)]
pub struct Build {
    #[serde(rename(deserialize = "type"))]
    pub build_type: String,
    pub ci_url: Option<String>,
    pub epoch: Option<u32>,
    pub nvr: String,
    pub release_id: Option<u32>,
    pub signed: bool,
}

/// This struct represents one comment against a specific update, along with its associated bug and
/// test case feedback.
#[derive(Debug, Deserialize)]
pub struct Comment {
    pub author: Option<String>,
    pub bug_feedback: Vec<BugFeedback>,
    pub id: u32,
    pub karma: Karma,
    pub karma_critpath: Karma,
    pub testcase_feedback: Vec<TestCaseFeedback>,
    pub text: String,
    pub timestamp: String,
    pub update: Option<Update>,
    pub update_id: u32,
    pub update_title: Option<String>,
    pub user: User,
    pub user_id: u32,
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
    pub build_id: u32,
    pub expiration_date: String,
    pub expired_date: Option<String>,
    pub notes: String,
    pub nvr: String,
    pub submission_date: String,
    pub submitter: User,
    pub submitter_id: u32,
}

/// This struct represents a specific fedora package.
#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub package_type: String,
    pub requirements: Option<String>,
}

/// This struct represents a fedora release as present in the bodhi database. This includes variants
/// (Modular, Container, Flatpak), identified with the "C", "F", and "M" suffixes.
#[derive(Debug, Deserialize)]
pub struct Release {
    pub branch: String,
    pub candidate_tag: String,
    pub composed_by_bodhi: bool,
    pub dist_tag: String,
    pub id_prefix: String,
    pub long_name: String,
    pub mail_template: String,
    pub name: FedoraRelease,
    pub override_tag: String,
    pub pending_signing_tag: String,
    pub pending_stable_tag: String,
    pub pending_testing_tag: String,
    pub stable_tag: String,
    pub state: String,
    pub testing_tag: String,
    pub version: String,
}

/// This struct represents a specific test case as associated with a given test case feedback and
/// update.
#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub package: Option<Package>,
    pub package_id: u32,
}

/// This struct represents an update feedback item associated with a specific test case.
#[derive(Debug, Deserialize)]
pub struct TestCaseFeedback {
    pub comment_id: Option<u32>,
    pub karma: Karma,
    pub testcase: TestCase,
    pub testcase_id: u32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UpdateID {
    ID(u32),
    Alias(String),
}

/// This struct represents a bodhi update, with associated items: bugs, builds, comments, release,
/// status, submitter, etc.
#[derive(Debug, Deserialize)]
pub struct Update {
    pub alias: String,
    pub autokarma: bool,
    pub bugs: Vec<Bug>,
    pub builds: Vec<Build>,
    pub close_bugs: bool,
    pub comments: Option<Vec<Comment>>,
    pub content_type: Option<ContentType>,
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
    #[serde(rename(deserialize = "old_updateid"))]
    pub old_update_id: Option<UpdateID>,
    pub pushed: bool,
    pub release: Release,
    pub request: Option<UpdateRequest>,
    pub require_bugs: bool,
    pub require_testcases: bool,
    pub requirements: Option<String>,
    pub severity: UpdateSeverity,
    pub stable_karma: Option<i32>,
    pub status: UpdateStatus,
    pub submitter: Option<String>,
    pub suggest: UpdateSuggestion,
    pub test_cases: Option<Vec<TestCase>>,
    pub test_gating_status: Option<String>,
    pub title: String,
    pub unstable_karma: Option<i32>,
    #[serde(rename(deserialize = "updateid"))]
    pub update_id: Option<UpdateID>,
    #[serde(rename(deserialize = "type"))]
    pub update_type: UpdateType,
    pub url: String,
    pub user: User,
}

/// This struct represents one fedora user that bodhi is aware of.
#[derive(Debug, Deserialize)]
pub struct User {
    pub avatar: String,
    pub email: Option<String>,
    pub groups: Vec<Group>,
    pub id: u32,
    pub name: String,
    pub openid: String,
}
