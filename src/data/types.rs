use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use fedora::url::Url;
use serde::{Deserialize, Serialize};

use super::dates::*;
use super::enums::*;
use super::release::FedoraRelease;

/// data type that represents a BugZilla bug that is associated with an update
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
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
        let title = match &self.title {
            Some(title) => title.as_str(),
            None => "(None)",
        };

        writeln!(f, "Bug {}:", self.bug_id)?;
        writeln!(f, "Title: {title}")?;
        writeln!(f, "URL:   {}", self.url())?;

        Ok(())
    }
}

impl Bug {
    /// construct the Red Hat BugZilla (RHBZ) URL from this [`Bug`] from its ID
    pub fn url(&self) -> Url {
        Url::parse(&format!("https://bugzilla.redhat.com/show_bug.cgi?id={}", self.bug_id))
            .expect("Failed to parse the hard-coded URL, this should not happen.")
    }
}


/// data type that represents a feedback item for a bug that is associated with an update
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct BugFeedback {
    /// bug this feedback is associated with
    pub bug: Option<Bug>,
    /// ID of the bug that this feedback is associated with
    pub bug_id: u32,
    /// ID of the comment that this feedback is associated with
    pub comment_id: Option<u32>,
    /// feedback karma (positive, neutral, negative)
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


/// data type that represents a koji build that is associated with an update
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Build {
    /// Epoch value of this build (`None` if unspecified)
    pub epoch: Option<u32>,
    /// NVR (Name-Version-Release) string of this build
    pub nvr: String,
    /// release ID of the release that this build is associated with
    pub release_id: Option<u32>,
    /// flag to indicate whether this build has been signed yet
    pub signed: bool,
    /// build type (RPM, container, flatpak, module)
    #[serde(rename = "type")]
    pub build_type: ContentType,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Build {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "Build {}", &self.nvr)?;
        writeln!(f, "Type:  {}", self.build_type)?;
        writeln!(
            f,
            "Epoch: {}",
            match self.epoch {
                Some(epoch) => epoch.to_string(),
                None => "(None)".to_string(),
            }
        )?;
        Ok(())
    }
}


/// data type that represents a comment on an update (including bug and test case feedback)
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Comment {
    // author of the comment (username), only provided for backwards compatibility
    #[deprecated(since = "2.0.0")]
    author: Option<String>,
    /// list of bug feedback items
    pub bug_feedback: Vec<BugFeedback>,
    /// numerical ID of this comment
    pub id: u32,
    /// karma feedback associated with this comment
    pub karma: Karma,
    // feedback associated with "critpath" checks
    #[deprecated(since = "2.0.0")]
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
    // alias of the update this comment is associated with
    // (only provided for backwards compatibility)
    #[deprecated(since = "2.0.0")]
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
        writeln!(f, "Comment by {}", &self.user.name)?;
        writeln!(f, "{}", &self.text)?;
        writeln!(f, "Submitted: {}", &self.timestamp)?;
        writeln!(f, "Karma:     {}", self.karma)?;

        Ok(())
    }
}


/// data type that represents a (running) compose for an "updates" or "updates-testing" repository
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Compose {
    /// string of JSON-formatted checkpoint data for the compose
    pub checkpoints: String,
    /// type of the contained contents (RPMs, containers, flatpaks, modules)
    pub content_type: Option<ContentType>,
    /// date & time when this compose was triggered
    #[serde(with = "bodhi_date_format")]
    pub date_created: BodhiDate,
    /// error message in case of failures (empty string if no errors have occurred yet)
    pub error_message: Option<String>,
    /// release this compose is associated with
    pub release: Option<Release>,
    /// numerical ID of the release this compose is associated with
    pub release_id: u32,
    /// target of the compose:
    ///
    /// - stable: "updates" repository
    /// - testing: "updates-testing" repository
    pub request: ComposeRequest,
    /// flag to indicate whether this compose contains security updates
    pub security: bool,
    /// current state of the compose
    pub state: ComposeState,
    /// date & time when the compose status was last updated
    #[serde(with = "bodhi_date_format")]
    pub state_date: BodhiDate,
    /// list of summaries for the contained updates (contains update aliases and titles)
    pub update_summary: Vec<UpdateSummary>,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Compose {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "Compose for {release} / {request} ({content_type})",
            release = match &self.release {
                Some(release) => release.name.to_string(),
                None => "(None)".to_string(),
            },
            request = &self.request,
            content_type = match &self.content_type {
                Some(content_type) => content_type.to_string(),
                None => "(None)".to_string(),
            }
        )?;

        writeln!(f, "Created: {}", &self.date_created)?;
        writeln!(f, "Status:  {}", self.state)?;

        Ok(())
    }
}


/// data type that represents a group of users in the fedora accounts system (FAS)
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Group {
    /// name of the group
    pub name: String,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}


/// data type that represents a buildroot override and its associated koji build
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Override {
    /// koji build that is associated with this buildroot override
    pub build: Build,
    /// build ID of the koji build that is associated with this buildroot override
    pub build_id: u32,
    /// date & time when this buildroot override will expire
    #[serde(with = "bodhi_date_format")]
    pub expiration_date: BodhiDate,
    /// date & time when this buildroot override has expired
    #[serde(with = "option_bodhi_date_format")]
    pub expired_date: Option<BodhiDate>,
    /// notes associated with this buildroot override
    pub notes: String,
    /// NVR (Name-Version-Release) string of the build that is associated with this buildroot
    /// override
    pub nvr: String,
    /// date & time when this buildroot override was submitted
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
        let expired_date = match &self.expired_date {
            Some(date) => date.to_string(),
            None => "no".to_string(),
        };

        writeln!(f, "Buildroot override for {}", &self.nvr)?;
        writeln!(f, "{}", &self.notes)?;
        writeln!(f, "Submitter:       {}", &self.submitter.name)?;
        writeln!(f, "Submission date: {}", &self.submission_date)?;
        writeln!(f, "Expiration date: {}", &self.expiration_date)?;
        writeln!(f, "Expired:         {}", &expired_date)?;

        Ok(())
    }
}


/// data type that represents a package (or other distributable content) known to bodhi
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Package {
    /// unique identifier of the (source) package (or container, flatpak, or module, as appropriate)
    pub name: String,
    /// type of the associated contents (RPM package, container image, flatpak image, DNF module)
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


/// data type that represents a release (or release variant, based on content type) known to bodhi
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Release {
    /// name of the dist-git branch that is associated with this release
    pub branch: String,
    /// name of the koji tag for update candidates
    pub candidate_tag: String,
    /// flag to indicate whether this release is composed by bodhi itself
    pub composed_by_bodhi: bool,
    /// optional list of running composes for this release
    #[deprecated(
        since = "2.0.1",
        note = "The `composes` field was dropped from serialized `Release` objects with bodhi server versions 6.0 and later. It is only kept for backwards compatibility, but will in the future always have a value of `None` when deserializing JSON server responses."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composes: Option<Vec<Compose>>,
    /// flag to indicate whether updates should automatically be created for this release
    pub create_automatic_updates: Option<bool>,
    /// value of the RPM `%{?dist}` tag on this release
    pub dist_tag: String,
    /// update alias prefix for this release (`FEDORA{-EPEL,}{-CONTAINER,-FLATPAK,-MODULAR,}`)
    pub id_prefix: String,
    /// long name of this release
    pub long_name: String,
    /// name of the email template for errata
    pub mail_template: String,
    /// short identifier of this release
    pub name: FedoraRelease,
    /// package manager that is used on this release (parsed into [`PackageManager`] variants)
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
    /// current state of this release (parsed into [`ReleaseState`] variants)
    pub state: ReleaseState,
    /// name of the repository that is used for testing updates
    pub testing_repository: Option<String>,
    /// name of the tag for builds that have been pushed to testing
    pub testing_tag: String,
    /// Fedora version string corresponding to this release
    pub version: String,
    /// end-of-life date of this release in the format `YYYY-MM-DD`
    pub eol: Option<String>,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "Release {}:", &self.name)?;
        writeln!(f, "State:               {}", &self.state)?;
        writeln!(f, "Branch:              {}", &self.branch)?;
        writeln!(f, "Candidate tag:       {}", &self.candidate_tag)?;
        writeln!(f, "Override tag:        {}", &self.override_tag)?;
        writeln!(f, "Pending signing tag: {}", &self.pending_signing_tag)?;
        writeln!(f, "Pending stable tag:  {}", &self.pending_stable_tag)?;
        writeln!(f, "Pending testing tag: {}", &self.pending_testing_tag)?;
        writeln!(f, "Stable tag:          {}", &self.stable_tag)?;
        writeln!(f, "Testing tag:         {}", &self.testing_tag)?;

        Ok(())
    }
}


/// data type that represents a test case that is associated with a package
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct TestCase {
    /// name of this test case
    pub name: String,
    /// package that this test case is associated with
    pub package: Option<Package>,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for TestCase {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let package = match &self.package {
            Some(package) => &package.name,
            None => "(None)",
        };

        write!(
            f,
            "Test Case '{name}' for package {package}",
            name = &self.name,
            package = &package
        )
    }
}

impl TestCase {
    /// construct the Fedora Project Wiki URL for this [`TestCase`] from its name
    pub fn url(&self) -> Url {
        Url::parse(&format!(
            "https://fedoraproject.org/wiki/{}",
            self.name.replace(' ', "_")
        ))
        .expect("Failed to parse the hard-coded URL, this should not happen.")
    }
}


/// data type that represents a feedback item for a test case that is associated with an update
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct TestCaseFeedback {
    /// ID of the comment that this feedback is associated with
    pub comment_id: Option<u32>,
    /// feedback karma (positive, neutral, negative)
    pub karma: Karma,
    /// test case that this feedback is associated with
    pub testcase: TestCase,
    /// ID of the test case that this feedback is associated with
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


/// data type that represents an update
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Update {
    /// user-visible, human-readable update alias (`FEDORA-2019-1A2BB23E`)
    pub alias: String,
    /// flag to indicate whether this update will be pushed to stable automatically (based on karma)
    pub autokarma: bool,
    /// flag to indicate whether this update will be pushed to stable automatically (based on time)
    pub autotime: bool,
    /// list of bugs that are associated with this update
    pub bugs: Vec<Bug>,
    /// list of builds that are associated with this update
    pub builds: Vec<Build>,
    /// flag to indicate whether bugs will be closed when this update is pushed to stable
    pub close_bugs: bool,
    /// list of comments that are associated with this update
    pub comments: Option<Vec<Comment>>,
    /// currently running compose that this update is included in
    pub compose: Option<Compose>,
    /// type of the contained contents (RPMs, containers, flatpaks, modules)
    pub content_type: Option<ContentType>,
    /// flag to indicate whether this update contains packages from the "critical path"
    pub critpath: bool,
    /// last date & time when this update has been approved
    #[deprecated(
        since = "2.0.0",
        note = "`date_approved` is an unused field: <https://github.com/fedora-infra/bodhi/issues/4171>"
    )]
    #[serde(with = "option_bodhi_date_format")]
    pub date_approved: Option<BodhiDate>,
    /// date & time when this update was modified
    #[serde(with = "option_bodhi_date_format")]
    pub date_modified: Option<BodhiDate>,
    /// date & time when this update was pushed
    #[serde(with = "option_bodhi_date_format")]
    pub date_pushed: Option<BodhiDate>,
    /// date & time when this update was pushed to stable
    #[serde(with = "option_bodhi_date_format")]
    pub date_stable: Option<BodhiDate>,
    /// date & time when this update was submitted
    #[serde(with = "option_bodhi_date_format")]
    pub date_submitted: Option<BodhiDate>,
    /// date & time when this update was pushed to testing
    #[serde(with = "option_bodhi_date_format")]
    pub date_testing: Option<BodhiDate>,
    /// displayed "pretty" name of this update
    pub display_name: String,
    /// koji side tag that this update was created from
    pub from_tag: Option<String>,
    /// current total of feedback karma values
    pub karma: Option<i32>,
    /// flag indicating whether this update can be edited
    pub locked: bool,
    /// flag indicating whether the update satisfies test requirements
    pub meets_testing_requirements: bool,
    /// notes / text that is associated with this update
    pub notes: String,
    /// flag indicating whether this update has already been pushed
    pub pushed: bool,
    /// release that this update was submitted for
    pub release: Release,
    /// currently requested new update status
    pub request: Option<UpdateRequest>,
    /// flag to specify whether feedback for bugs is required when adding karma to the total
    pub require_bugs: bool,
    /// flag to specify whether feedback for test cases is required when adding karma to the total
    pub require_testcases: bool,
    /// comma- or space-separated list of required gating test results
    pub requirements: Option<String>,
    /// severity of this update
    pub severity: UpdateSeverity,
    /// minimum number of days this update has to stay in the [`UpdateStatus::Testing`] state
    pub stable_days: Option<u32>,
    /// stable karma threshold for this update
    pub stable_karma: Option<i32>,
    /// current state of this update
    pub status: UpdateStatus,
    /// suggested action to take after installing this update
    pub suggest: UpdateSuggestion,
    /// list test cases that is  associated with this update
    pub test_cases: Option<Vec<TestCase>>,
    /// current greenwave gating status
    ///
    /// If this value is `None`, greenwave was not yet enabled when this update was created.
    pub test_gating_status: Option<TestGatingStatus>,
    /// title of this update (automatically generated from build NVRs if `display_name` is `None`)
    pub title: String,
    /// unstable karma threshold for this update
    pub unstable_karma: Option<i32>,
    // updateid is only provided for backwards compatibility with bodhi 1
    #[deprecated(since = "2.0.0")]
    #[serde(rename = "updateid")]
    update_id: Option<UpdateID>,
    /// type of this update
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    /// public URL of this update
    pub url: String,
    /// user who first created this update
    pub user: User,
    /// SHA-1 hash of the sorted, space-separated NVRs of the included builds
    pub version_hash: String,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for Update {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let builds = if !self.builds.is_empty() {
            self.builds
                .iter()
                .map(|b| b.nvr.as_str())
                .collect::<Vec<&str>>()
                .join("\n")
        } else {
            String::from("(None)")
        };

        let bugs = if !self.bugs.is_empty() {
            self.bugs
                .iter()
                .map(|b| b.bug_id.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        } else {
            String::from("(None)")
        };

        let test_cases = match &self.test_cases {
            Some(test_cases) => {
                if !test_cases.is_empty() {
                    test_cases
                        .iter()
                        .map(|t| t.name.as_str())
                        .collect::<Vec<&str>>()
                        .join(" ")
                } else {
                    "(None)".to_string()
                }
            },
            None => "(None)".to_string(),
        };

        writeln!(f, "Update {}:", &self.alias)?;
        writeln!(f, "{}", &self.notes)?;
        writeln!(f)?;
        writeln!(f, "State:         {}", self.status)?;
        writeln!(f, "Submitter:     {}", &self.user.name)?;
        writeln!(f)?;
        writeln!(f, "Builds:")?;
        writeln!(f, "{}", &builds)?;
        writeln!(f)?;
        writeln!(f, "Bugs:       {}", &bugs)?;
        writeln!(f, "Test Cases: {}", &test_cases)?;

        Ok(())
    }
}


/// data type that represents an update summary
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UpdateSummary {
    /// update alias that uniquely identifies the update
    pub alias: String,
    /// user-defined or automatically generated update title
    pub title: String,
}

impl Display for UpdateSummary {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.alias, self.title)
    }
}


/// data type that represents a user in the Fedora Accounts System (FAS) who is known to bodhi
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct User {
    /// URL of the [libravatar](https://www.libravatar.org/) avatar for this user
    pub avatar: Option<String>,
    /// E-Mail address associated with this user (if public according to their account settings)
    pub email: Option<String>,
    /// list of groups this user is a member of
    pub groups: Vec<Group>,
    /// user ID that is associated with this user
    pub id: u32,
    /// unique FAS username of this user
    pub name: String,
    /// OpenID identity that is associated with the user
    pub openid: Option<String>,

    /// catch-all for fields that are not explicitly deserialized
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let email = match &self.email {
            Some(email) => email.as_str(),
            None => "(None)",
        };

        let groups: String = self
            .groups
            .iter()
            .map(|g| g.name.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        writeln!(f, "User {}:", &self.name)?;
        writeln!(f, "E-Mail: {email}")?;
        writeln!(f, "Groups: {}", &groups)?;

        Ok(())
    }
}
