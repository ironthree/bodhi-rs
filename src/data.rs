//! This module contains the data types that are needed to deserialize bodhi server responses which
//! are also public outside this crate. Some internal data types for queries are implemented in the
//! corresponding query module.
//!
//! Additionally, the bodhi REST API has some arguments that accept Strings, but only from a limited
//! set of enumerated values. To abstract this, the corresponding query filters accept some of the
//! enum types defined here, instead of the String arguments directly.

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// base URL of the fedora bodhi instance
pub const FEDORA_BODHI_URL: &str = "https://bodhi.fedoraproject.org";

/// base URL of the fedora bodhi staging instance
pub const FEDORA_BODHI_STG_URL: &str = "https://bodhi.stg.fedoraproject.org";

/// date format internally used by bodhi
pub const BODHI_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

// https://serde.rs/custom-date-format.html
#[allow(dead_code)]
mod bodhi_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(super::BODHI_DATE_FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, super::BODHI_DATE_FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

// https://github.com/serde-rs/serde/issues/1444#issuecomment-447546415
#[allow(dead_code)]
mod option_bodhi_date_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(ref dt) => super::bodhi_date_format::serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(with = "super::bodhi_date_format")] DateTime<Utc>);

        let v: Option<Wrapper> = Deserialize::deserialize(deserializer)?;
        Ok(v.map(|Wrapper(a)| a))
    }
}

/// This enum represents a "Karma" value, which is either a positive (+1), neutral (±0), or negative
/// (-1) feedback for an update, and is associated with a [`Comment`](struct.Comment.html), and
/// possibly also a [`TestCaseFeedback`](struct.TestCase.html) or a
/// [`BugFeedback`](struct.BugFeedback.html).
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(i8)]
pub enum Karma {
    /// positive feedback
    Positive = 1,
    /// neutral / informational feedback
    Neutral = 0,
    /// negative feedback
    Negative = -1,
}

/// This enum represents a fedora release.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Serialize)]
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
    #[serde(rename = "EPEL-8")]
    EPEL8,
    #[serde(rename = "EPEL-8M")]
    EPEL8M,
    #[serde(rename = "EPEL-7")]
    EPEL7,
    #[serde(rename = "EL-6")]
    EL6,
    #[serde(rename = "EL-5")]
    EL5,
}

/// This enum represents the type of a bodhi update, of a package, and of builds.
#[derive(Debug, Deserialize, Serialize)]
pub enum ContentType {
    /// "base" content type (seems to be unused)
    #[serde(rename = "base")]
    Base,
    /// tag for container image updates
    #[serde(rename = "container")]
    Container,
    /// tag for flatpak updates
    #[serde(rename = "flatpak")]
    Flatpak,
    /// tag for module updates
    #[serde(rename = "module")]
    Module,
    /// tag for traditional RPM package updates
    #[serde(rename = "rpm")]
    RPM,
}

/// This enum represents a requested state change of an update.
#[derive(Debug, Deserialize, Serialize)]
pub enum UpdateRequest {
    /// request for an update to get "batched" for the next stable push (no longer used)
    #[serde(rename = "batched")]
    Batched,
    /// request for an update to be marked as "obsolete" (usually when another update supersedes it)
    #[serde(rename = "obsolete")]
    Obsolete,
    /// request for the update to be "revoked" or removed
    #[serde(rename = "revoke")]
    Revoke,
    /// request for the update to get pushed to stable
    #[serde(rename = "stable")]
    Stable,
    /// request for the update to get pushed to testing
    #[serde(rename = "testing")]
    Testing,
    /// request for the update to get "unpushed" (removed) from testing
    #[serde(rename = "unpush")]
    Unpush,
}

/// This enum represents the associated severity of a bodhi update.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Serialize)]
pub enum UpdateSeverity {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "unspecified")]
    Unspecified,
    #[serde(rename = "urgent")]
    Urgent,
}

/// This enum represents the current state of a bodhi update.
#[derive(Debug, Deserialize, Serialize)]
pub enum UpdateStatus {
    /// tag for updates that have been obsoleted by another update
    #[serde(rename = "obsolete")]
    Obsolete,
    /// tag for updates that are pending for either testing or stable
    #[serde(rename = "pending")]
    Pending,
    /// tag for updates that are still being processed
    #[serde(rename = "processing")]
    Processing,
    /// tag for updates that are associated with an active side tag
    #[serde(rename = "side_tag_active")]
    SideTagActive,
    /// tag for updates that are associated with an expired side tag
    #[serde(rename = "side_tag_expired")]
    SideTagExpired,
    /// tag for updates that have been pushed to stable
    #[serde(rename = "stable")]
    Stable,
    /// tag for updates that have been pushed to testing
    #[serde(rename = "testing")]
    Testing,
    /// tag for updates that have been "unpushed" from testing
    #[serde(rename = "unpushed")]
    Unpushed,
}

/// This enum represents the associated suggested action for a bodhi update.
#[derive(Debug, Deserialize, Serialize)]
pub enum UpdateSuggestion {
    /// recommendation to log out for the update to get applied
    #[serde(rename = "logout")]
    Logout,
    /// recommendation to reboot for the update to get applied
    #[serde(rename = "reboot")]
    Reboot,
    /// no recommendation
    #[serde(rename = "unspecified")]
    Unspecified,
}

/// This enum represents the type of a bodhi update.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Serialize)]
pub enum UpdateType {
    #[serde(rename = "bugfix")]
    BugFix,
    #[serde(rename = "security")]
    Enhancement,
    #[serde(rename = "newpackage")]
    NewPackage,
    #[serde(rename = "enhancement")]
    Security,
    #[serde(rename = "unspecified")]
    Unspecified,
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
    /// bug this feedback is associated with
    pub bug: Option<Bug>,
    /// ID of the bug this feedback is associated with
    pub bug_id: u32,
    /// ID of the comment that this feedback is associated with
    pub comment_id: Option<u32>,
    /// feedback (positive, neutral, negative)
    pub karma: Karma,
}

/// This struct represents a specific koji build that bodhi is aware of.
#[derive(Debug, Deserialize)]
pub struct Build {
    /// type of this build; one of: `container`, `flatpak`, `module`, `rpm`
    #[serde(rename = "type")]
    pub build_type: ContentType,
    /// URL pointing to the results of CI tests
    pub ci_url: Option<String>,
    /// epoch associated with this build
    pub epoch: Option<u32>,
    /// (Name-Version-Release) value for this build
    pub nvr: String,
    /// release ID of the release this build is associated with
    pub release_id: Option<u32>,
    /// flag to indicate whether this build has been signed yet
    pub signed: bool,
}

/// This struct represents one comment against a specific update, along with its associated bug and
/// test case feedback.
#[derive(Debug, Deserialize)]
pub struct Comment {
    /// author of the comment (username)
    pub author: Option<String>,
    /// list of bug feedback items
    pub bug_feedback: Vec<BugFeedback>,
    /// numerical ID of this comment
    pub id: u32,
    /// feedback associated with this comment
    pub karma: Karma,
    /// feedback associated with "critpath" checks
    pub karma_critpath: Karma,
    /// list of test case feedback items
    pub testcase_feedback: Vec<TestCaseFeedback>,
    /// text of the comment
    pub text: String,
    /// date & time this comment was published
    #[serde(with = "bodhi_date_format")]
    pub timestamp: DateTime<Utc>,
    /// update this comment is associated with
    pub update: Option<Update>,
    /// ID of the update this comment is associated with
    pub update_id: u32,
    /// title of the update this comment is associated with
    pub update_title: Option<String>,
    /// user who submitted this comment
    pub user: User,
    /// user ID of the user who submitted this comment
    pub user_id: u32,
}

/// This struct represents a group from the fedora accounts system (FAS).
#[derive(Debug, Deserialize)]
pub struct Group {
    /// name of the group
    pub name: String,
}

/// This struct represents a buildroot override, along with the associated build.
#[derive(Debug, Deserialize)]
pub struct Override {
    /// build associated with this buildroot override
    pub build: Build,
    /// build ID of the build associated with this buildroot override
    pub build_id: u32,
    /// date & time this buildroot override will expire
    #[serde(with = "bodhi_date_format")]
    pub expiration_date: DateTime<Utc>,
    /// date & time this buildroot override has expired
    #[serde(with = "option_bodhi_date_format")]
    pub expired_date: Option<DateTime<Utc>>,
    /// public notes associated with this buildroot override
    pub notes: String,
    /// value of (Name-Version-Release) of the build associated with this buildroot override
    pub nvr: String,
    /// date & time this buildroot override was submitted
    #[serde(with = "bodhi_date_format")]
    pub submission_date: DateTime<Utc>,
    /// user who submitted this buildroot override
    pub submitter: User,
    /// user ID of the user who submitted this buildroot override
    pub submitter_id: u32,
}

/// This struct represents a specific fedora package (or another distributable unit)
#[derive(Debug, Deserialize)]
pub struct Package {
    /// unique name of the (source) package (or container, flatpak, or module, as appropriate)
    pub name: String,
    /// content type; one of: `rpm`, `container`, `flatpak`, `module`
    #[serde(rename = "type")]
    pub package_type: ContentType,
}

/// This enum represents the state of a release.
#[derive(Debug, Deserialize)]
pub enum ReleaseState {
    /// release has been archived after it has reached its EOL
    #[serde(rename = "archived")]
    Archived,
    /// release is currently supported
    #[serde(rename = "current")]
    Current,
    /// release is in development
    #[serde(rename = "pending")]
    Pending,
}

/// This struct represents a fedora release as present in the bodhi database. This includes variants
/// (Modular, Container, Flatpak), identified with the "C", "F", and "M" suffixes.
#[derive(Debug, Deserialize)]
pub struct Release {
    /// dist-git branch for this release
    pub branch: String,
    /// name of the tag for update candidates
    pub candidate_tag: String,
    /// flag to indicate whether this release is composed by bodhi
    pub composed_by_bodhi: bool,
    /// value of the RPM `%{?dist}` tag on this release
    pub dist_tag: String,
    /// prefix for update aliases: one of `FEDORA{-EPEL,}{-CONTAINER,-FLATPAK,-MODULAR,}`
    pub id_prefix: String,
    /// long name of this release
    pub long_name: String,
    /// name of the email template for errata
    pub mail_template: String,
    /// short name of this release
    pub name: FedoraRelease,
    /// name of the tag for builds in buildroot overrides
    pub override_tag: String,
    /// name of the tag for builds that are pending to be signed
    pub pending_signing_tag: String,
    /// name of the tag for builds that are pending to be pushed to stable
    pub pending_stable_tag: String,
    /// name of the tag for builds that are pending to be pushed to testing
    pub pending_testing_tag: String,
    /// name of the tag for builds that have been pushed to stable
    pub stable_tag: String,
    /// current state of this release; one of: `archived`, `current`, `pending`
    pub state: ReleaseState,
    /// name of the tag for builds that have been pushed to testing
    pub testing_tag: String,
    /// version string of this release
    pub version: String,
}

/// This struct represents a specific test case as associated with a package.
#[derive(Debug, Deserialize)]
pub struct TestCase {
    /// name of this test case
    pub name: String,
    /// package this test case is associated with
    pub package: Option<Package>,
    /// ID of the package this test case is associated with
    pub package_id: u32,
}

/// This struct represents an update feedback item associated with a specific test case.
#[derive(Debug, Deserialize)]
pub struct TestCaseFeedback {
    /// ID of the comment this feedback is associated with
    pub comment_id: Option<u32>,
    /// karma feedback
    pub karma: Karma,
    /// test case this feedback is associated with
    pub testcase: TestCase,
    /// ID of the test case this feedback is associated with
    pub testcase_id: u32,
}

/// This enum represents the two possible ways to identify a fedora update:
/// - internal, numerical ID
/// - public, human-readable "alias" (`FEDORA-2019-1A2BB23E`)
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UpdateID {
    /// identified via numerical update ID
    ID(u32),
    /// identified via update alias
    Alias(String),
}

/// This struct represents a bodhi update, with associated items: bugs, builds, comments, release,
/// status, submitter, etc.
#[derive(Debug, Deserialize)]
pub struct Update {
    /// user-visible, human-readable update alias (FEDORA-2019-1A2BB23E)
    pub alias: String,
    /// flag to indicate whether this update can be pushed to stable automatically based on karma
    pub autokarma: bool,
    /// bugs associated with this update
    pub bugs: Vec<Bug>,
    /// builds associated with this update
    pub builds: Vec<Build>,
    /// flag to indicate whether bugs will be closed when this update is pushed to stable
    pub close_bugs: bool,
    /// list of comments associated with this update
    pub comments: Option<Vec<Comment>>,
    /// content type of this update (RPM, Module, Flatpak, Container)
    pub content_type: Option<ContentType>,
    /// flag to indicate whether this update contains packages from the "critical path"
    pub critpath: bool,
    /// date & time when this update has last been approved
    #[serde(with = "option_bodhi_date_format")]
    pub date_approved: Option<DateTime<Utc>>,
    /// date & time when this update has last been modified
    #[serde(with = "option_bodhi_date_format")]
    pub date_modified: Option<DateTime<Utc>>,
    /// date & time when this update has last been pushed
    #[serde(with = "option_bodhi_date_format")]
    pub date_pushed: Option<DateTime<Utc>>,
    /// date & time when this update has last been pushed to stable
    #[serde(with = "option_bodhi_date_format")]
    pub date_stable: Option<DateTime<Utc>>,
    /// date & time when this update has last been submitted
    #[serde(with = "option_bodhi_date_format")]
    pub date_submitted: Option<DateTime<Utc>>,
    /// date & time when this update has last been pushed to testing
    #[serde(with = "option_bodhi_date_format")]
    pub date_testing: Option<DateTime<Utc>>,
    /// displayed name of this update
    pub display_name: String,
    /// greenwave status summary string
    pub greenwave_summary_string: Option<String>,
    /// comma- or space-separated list of unsatisfied greenwave gating requirements
    pub greenwave_unsatisfied_requirements: Option<String>,
    /// current karma total
    pub karma: Option<i32>,
    /// flag indicating whether this update can be edited
    pub locked: bool,
    /// flag indicating whether the update satisfies requirements
    pub meets_testing_requirements: bool,
    /// public notes associated with this update
    pub notes: String,
    /// when editing an existing update, this ID has to be specified
    #[serde(rename = "old_updateid")]
    pub old_update_id: Option<UpdateID>,
    /// flag indicating whether this update has already been pushed
    pub pushed: bool,
    /// release this update was submitted for
    pub release: Release,
    /// currently requested new update status
    pub request: Option<UpdateRequest>,
    /// flag to specify whether feedback for bugs is required when counting karma
    pub require_bugs: bool,
    /// flag to specify whether feedback for test cases is required when counting karma
    pub require_testcases: bool,
    /// comma- or space-separated list of required taskotron test results
    pub requirements: Option<String>,
    /// severity of this update
    pub severity: UpdateSeverity,
    /// stable karma threshold set for this update
    pub stable_karma: Option<i32>,
    /// current status of this update
    pub status: UpdateStatus,
    /// username of the update submitter
    pub submitter: Option<String>,
    /// suggested action to take after installing this update
    pub suggest: UpdateSuggestion,
    /// list test cases associated with this update
    pub test_cases: Option<Vec<TestCase>>,
    /// greenwave gating status; one of: `failed`, `greenwave_failed`, `ignored`, `passed`
    pub test_gating_status: Option<String>,
    /// title of this update
    pub title: String,
    /// unstable karma threshold set for this update
    pub unstable_karma: Option<i32>,
    /// update ID associated with this update (either alias or numeric ID)
    #[serde(rename = "updateid")]
    pub update_id: Option<UpdateID>,
    /// type of this update
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    /// public URL of this update
    pub url: String,
    /// user who created this update
    pub user: User,
}

/// This struct represents one fedora user that bodhi is aware of.
#[derive(Debug, Deserialize)]
pub struct User {
    /// URL of the [libravatar](https://www.libravatar.org/) avatar for this user
    pub avatar: String,
    /// E-Mail address associated with this user (if set to public)
    pub email: Option<String>,
    /// group memberships for this user
    pub groups: Vec<Group>,
    /// user ID associated with this user
    pub id: u32,
    /// username identifying this user
    pub name: String,
    /// OpenID identity associated with the user
    pub openid: String,
}
