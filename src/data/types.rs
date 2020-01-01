use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::Deserialize;

use super::dates::*;
use super::enums::*;

/// This struct represents a specific BugZilla bug that is associated with an update.
#[derive(Debug, Deserialize)]
pub struct Bug {
    /// bug ID in the BugZilla system: <https://bugzilla.redhat.com/show_bug.cgi?id={bug_id}>
    pub bug_id: u32,
    /// flag to indicate whether this bug has been tagged as a parent / tracking bug
    pub parent: bool,
    /// flag to indicate whether this bug has been tagged as a `Security` issue
    pub security: bool,
    /// title of the bug in BugZilla
    pub title: Option<String>,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Bug {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Bug {bug_id}: {title}",
            bug_id = self.bug_id,
            title = match &self.title {
                Some(title) => title.as_str(),
                None => "(None)",
            }
        )
    }
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

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for BugFeedback {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{bug_id}: {karma}", bug_id = self.bug_id, karma = self.karma)
    }
}


/// This struct represents a specific koji build that bodhi is aware of.
#[derive(Debug, Deserialize)]
pub struct Build {
    /// epoch associated with this build
    pub epoch: Option<u32>,
    /// (Name-Version-Release) value for this build
    pub nvr: String,
    /// release ID of the release this build is associated with
    pub release_id: Option<u32>,
    /// flag to indicate whether this build has been signed yet
    pub signed: bool,
    /// type of this build; one of: `container`, `flatpak`, `module`, `rpm`
    #[serde(rename = "type")]
    pub build_type: ContentType,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Build {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Build {nvr}:\n\
             Type:  {build_type}\n\
             Epoch: {epoch}",
            nvr = &self.nvr,
            build_type = self.build_type,
            epoch = match self.epoch {
                Some(epoch) => epoch.to_string(),
                None => "(None)".to_string(),
            }
        )
    }
}


/// This struct represents one comment against a specific update, along with its associated bug and
/// test case feedback.
#[derive(Debug, Deserialize)]
pub struct Comment {
    /// author of the comment (username), only provided for backwards compatibility
    author: Option<String>,
    /// list of bug feedback items
    pub bug_feedback: Vec<BugFeedback>,
    /// numerical ID of this comment
    pub id: u32,
    /// feedback associated with this comment
    pub karma: Karma,
    /// feedback associated with "critpath" checks (deprecated)
    karma_critpath: Karma,
    /// list of test case feedback items
    pub testcase_feedback: Vec<TestCaseFeedback>,
    /// text of the comment
    pub text: String,
    /// date & time this comment was published
    #[serde(with = "bodhi_date_format")]
    pub timestamp: BodhiDate,
    /// update this comment is associated with
    pub update: Option<Update>,
    /// ID of the update this comment is associated with
    pub update_id: u32,
    /// alias of the update this comment is for, only provided for backwards compatibility
    update_alias: Option<String>,
    /// user who submitted this comment
    pub user: User,
    /// user ID of the user who submitted this comment
    pub user_id: u32,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Comment by {author}:\n\
             {text}\n\
             \n\
             Submitted:      {timestamp}\n\
             Karma:          {karma}",
            author = &self.user.name,
            text = &self.text,
            timestamp = &self.timestamp,
            karma = self.karma,
        )
    }
}


/// This struct represents a currently running compose.
#[derive(Debug, Deserialize)]
pub struct Compose {
    /// map of active checkpoints for the compose
    pub checkpoints: Checkpoints,
    /// type of the contained contents (RPMs, containers, flatpaks, modules)
    pub content_type: Option<ContentType>,
    /// date & time this compose was triggered
    #[serde(with = "bodhi_date_format")]
    pub date_created: BodhiDate,
    /// error message in case of failure, else empty string
    pub error_message: String,
    /// release this compose is running for
    pub release: Option<Release>,
    /// ID of the release this compose is running for
    pub release_id: u32,
    /// request for the compose (stable or testing)
    pub request: ComposeRequest,
    /// flag to indicate whether a compose contains security updates
    pub security: bool,
    /// state of the compose
    pub state: ComposeStatus,
    /// date & time the compose status was last updated
    #[serde(with = "bodhi_date_format")]
    pub state_date: BodhiDate,
    /// list of summaries for the contained updates (with update alias and title)
    pub update_summary: Vec<UpdateSummary>,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Compose {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Compose for {release} / {request} ({content_type}):\n\
             Created: {created}\n\
             Status:  {status}",
            release = match &self.release {
                Some(release) => release.name.to_string(),
                None => "(None)".to_string(),
            },
            request = &self.request,
            content_type = match &self.content_type {
                Some(content_type) => content_type.to_string(),
                None => "(None)".to_string(),
            },
            created = &self.date_created,
            status = &self.state,
        )
    }
}


/// This struct represents a group from the fedora accounts system (FAS).
#[derive(Debug, Deserialize)]
pub struct Group {
    /// name of the group
    pub name: String,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{name}", name = &self.name)
    }
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
    pub expiration_date: BodhiDate,
    /// date & time this buildroot override has expired
    #[serde(with = "option_bodhi_date_format")]
    pub expired_date: Option<BodhiDate>,
    /// public notes associated with this buildroot override
    pub notes: String,
    /// value of (Name-Version-Release) of the build associated with this buildroot override
    pub nvr: String,
    /// date & time this buildroot override was submitted
    #[serde(with = "bodhi_date_format")]
    pub submission_date: BodhiDate,
    /// user who submitted this buildroot override
    pub submitter: User,
    /// user ID of the user who submitted this buildroot override
    pub submitter_id: u32,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Override {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Buildroot override for {nvr}:\n\
             {notes}\n\
             \n\
             Submitter:       {submitter}\n\
             Submission date: {submitted}\n\
             Expiration date: {expiration}\n\
             Expired:         {expired}",
            nvr = &self.nvr,
            notes = &self.notes,
            submitter = &self.submitter.name,
            submitted = &self.submission_date,
            expiration = &self.expiration_date,
            expired = match &self.expired_date {
                Some(date) => date.to_string(),
                None => "no".to_string(),
            },
        )
    }
}


/// This struct represents a specific fedora package (or another distributable unit)
#[derive(Debug, Deserialize)]
pub struct Package {
    /// unique name of the (source) package (or container, flatpak, or module, as appropriate)
    pub name: String,
    /// content type; one of: `rpm`, `container`, `flatpak`, `module`
    #[serde(rename = "type")]
    pub package_type: ContentType,
    /// test case requirements associated with this package
    pub requirements: Option<String>,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{name} ({package_type})",
            name = &self.name,
            package_type = self.package_type
        )
    }
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
    /// optional list of running composes for this release
    pub composes: Option<Vec<Compose>>,
    /// flag to indicate whether updates are automatically created for this release
    pub create_automatic_updates: Option<bool>,
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
    /// package manager in use on this release
    pub package_manager: PackageManager,
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
    /// name of the repository used for testing updates
    pub testing_repository: Option<String>,
    /// name of the tag for builds that have been pushed to testing
    pub testing_tag: String,
    /// version string of this release
    pub version: String,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Release {name}:\n\
             State:               {state}\n\
             Branch:              {branch}\n\
             Candidate tag:       {candidate_tag}\n\
             Override tag:        {override_tag}\n\
             Pending signing tag: {pending_signing_tag}\n\
             Pending stable tag:  {pending_stable_tag}\n\
             Pending testing tag: {pending_testing_tag}\n\
             Stable tag:          {stable_tag}\n\
             Testing tag:         {testing_tag}",
            name = &self.name,
            state = &self.state,
            branch = &self.branch,
            candidate_tag = &self.candidate_tag,
            override_tag = &self.override_tag,
            pending_signing_tag = &self.pending_signing_tag,
            pending_stable_tag = &self.pending_stable_tag,
            pending_testing_tag = &self.pending_testing_tag,
            stable_tag = &self.stable_tag,
            testing_tag = &self.testing_tag,
        )
    }
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

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for TestCase {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Test Case '{name}' for package {package}",
            name = &self.name,
            package = match &self.package {
                Some(package) => &package.name,
                None => "(None)",
            }
        )
    }
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

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for TestCaseFeedback {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{name}: {karma}", name = &self.testcase.name, karma = self.karma)
    }
}


/// This struct represents a bodhi update, with associated items: bugs, builds, comments, release,
/// status, submitter, etc.
#[derive(Debug, Deserialize)]
pub struct Update {
    /// user-visible, human-readable update alias (`FEDORA-2019-1A2BB23E`)
    pub alias: String,
    /// flag to indicate whether this update can be pushed to stable automatically based on karma
    pub autokarma: bool,
    /// flag to indicate whether this update can be pushed to stable automatically based on time
    pub autotime: bool,
    /// bugs associated with this update
    pub bugs: Vec<Bug>,
    /// builds associated with this update
    pub builds: Vec<Build>,
    /// flag to indicate whether bugs will be closed when this update is pushed to stable
    pub close_bugs: bool,
    /// list of comments associated with this update
    pub comments: Option<Vec<Comment>>,
    /// compose this update is currently part of
    pub compose: Option<Compose>,
    /// content type of this update (RPM, Module, Flatpak, Container)
    pub content_type: Option<ContentType>,
    /// flag to indicate whether this update contains packages from the "critical path"
    pub critpath: bool,
    /// date & time when this update has last been approved
    #[serde(with = "option_bodhi_date_format")]
    pub date_approved: Option<BodhiDate>,
    /// date & time when this update has last been modified
    #[serde(with = "option_bodhi_date_format")]
    pub date_modified: Option<BodhiDate>,
    /// date & time when this update has last been pushed
    #[serde(with = "option_bodhi_date_format")]
    pub date_pushed: Option<BodhiDate>,
    /// date & time when this update has last been pushed to stable
    #[serde(with = "option_bodhi_date_format")]
    pub date_stable: Option<BodhiDate>,
    /// date & time when this update has last been submitted
    #[serde(with = "option_bodhi_date_format")]
    pub date_submitted: Option<BodhiDate>,
    /// date & time when this update has last been pushed to testing
    #[serde(with = "option_bodhi_date_format")]
    pub date_testing: Option<BodhiDate>,
    /// displayed name of this update
    pub display_name: String,
    /// side tag this update was created from
    pub from_tag: Option<String>,
    /// current karma total
    pub karma: Option<i32>,
    /// flag indicating whether this update can be edited
    pub locked: bool,
    /// flag indicating whether the update satisfies requirements
    pub meets_testing_requirements: bool,
    /// public notes associated with this update
    pub notes: String,
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
    /// minimum of days this update has to stay in testing
    pub stable_days: Option<u32>,
    /// stable karma threshold set for this update
    pub stable_karma: Option<i32>,
    /// current status of this update
    pub status: UpdateStatus,
    /// suggested action to take after installing this update
    pub suggest: UpdateSuggestion,
    /// list test cases associated with this update
    pub test_cases: Option<Vec<TestCase>>,
    /// greenwave gating status; one of:
    /// `failed`, `greenwave_failed`, `ignored`, `passed`, `waiting`
    /// If this value is `None`, greenwave was not yet enabled when this update was created.
    pub test_gating_status: Option<TestGatingStatus>,
    /// title of this update (automatically computed from build NVRs if `display_name` is not set)
    pub title: String,
    /// unstable karma threshold set for this update
    pub unstable_karma: Option<i32>,
    /// updateid is only provided for backwards compatibility with bodhi 1
    #[serde(rename = "updateid")]
    update_id: Option<UpdateID>,
    /// type of this update
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    /// public URL of this update
    pub url: String,
    /// user who created this update
    pub user: User,
    /// SHA-1 hash of the sorted, space-separated NVRs of the included builds
    pub version_hash: String,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Update {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let builds = &self
            .builds
            .iter()
            .map(|b| b.nvr.as_str())
            .collect::<Vec<&str>>()
            .join("\n");

        let bugs = &self
            .bugs
            .iter()
            .map(|b| b.bug_id.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let test_cases = match &self.test_cases {
            Some(test_cases) => test_cases
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<&str>>()
                .join(" "),
            None => "(None)".to_string(),
        };

        write!(
            f,
            "Update {alias}:\n\
             {notes}\n\
             \n\
             State:         {state}\n\
             Submitter:     {submitter}\n\
             \n\
             Builds:\n\
             {builds}\n\
             \n\
             Bugs: {bugs}\n\
             Test Cases: {test_cases}\n\
             ",
            alias = &self.alias,
            notes = &self.notes,
            state = &self.status,
            submitter = &self.user.name,
            builds = &builds,
            bugs = &bugs,
            test_cases = &test_cases,
        )
    }
}


/// This struct wraps the short update summaries that are included in running
/// [`Compose`](struct.Compose.html)s.
#[derive(Debug, Deserialize)]
pub struct UpdateSummary {
    /// unique update alias identifying the update
    pub alias: String,
    /// user-defined, descriptive update title
    pub title: String,
}

impl Display for UpdateSummary {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.alias, self.title)
    }
}


/// This struct represents one fedora user that bodhi is aware of.
#[derive(Debug, Deserialize)]
pub struct User {
    /// URL of the [libravatar](https://www.libravatar.org/) avatar for this user
    pub avatar: Option<String>,
    /// E-Mail address associated with this user (if set to public)
    pub email: Option<String>,
    /// group memberships for this user
    pub groups: Vec<Group>,
    /// user ID associated with this user
    pub id: u32,
    /// username identifying this user
    pub name: String,
    /// OpenID identity associated with the user
    pub openid: Option<String>,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let groups: Vec<&str> = self.groups.iter().map(|g| g.name.as_str()).collect();

        write!(
            f,
            "User {name}:\n\
             E-Mail: {email}\n\
             Groups: {groups}",
            name = &self.name,
            email = match &self.email {
                Some(email) => email,
                None => "(None)",
            },
            groups = match groups.len() {
                0 => "(None)".to_string(),
                _ => groups.join(", "),
            },
        )
    }
}
