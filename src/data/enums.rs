use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::InvalidValueError;

// imports for intra-doc links
#[cfg(doc)]
use super::FedoraRelease;

/// valid `request` values for composes
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ComposeRequest {
    #[serde(rename = "stable")]
    Stable,
    #[serde(rename = "testing")]
    Testing,
}

impl Display for ComposeRequest {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            ComposeRequest::Stable => "stable",
            ComposeRequest::Testing => "testing",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for ComposeRequest {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "stable" => Ok(ComposeRequest::Stable),
            "testing" => Ok(ComposeRequest::Testing),
            _ => Err(InvalidValueError::new("ComposeRequest", value.to_owned())),
        }
    }
}

impl FromStr for ComposeRequest {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// valid `state` values for composes
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ComposeState {
    #[serde(rename = "cleaning")]
    Cleaning,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "notifying")]
    Notifying,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "punging")]
    Punging,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "signing_repo")]
    SigningRepo,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "syncing_repo")]
    SyncingRepo,
    #[serde(rename = "updateinfo")]
    UpdateInfo,
}

impl Display for ComposeState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            ComposeState::Cleaning => "cleaning",
            ComposeState::Failed => "failed",
            ComposeState::Initializing => "initializing",
            ComposeState::Notifying => "notifying",
            ComposeState::Pending => "pending",
            ComposeState::Punging => "punging",
            ComposeState::Requested => "requested",
            ComposeState::SigningRepo => "signing_repo",
            ComposeState::Success => "success",
            ComposeState::SyncingRepo => "syncing_repo",
            ComposeState::UpdateInfo => "updateinfo",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for ComposeState {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "cleaning" => Ok(ComposeState::Cleaning),
            "failed" => Ok(ComposeState::Failed),
            "initializing" => Ok(ComposeState::Initializing),
            "notifying" => Ok(ComposeState::Notifying),
            "pending" => Ok(ComposeState::Pending),
            "punging" => Ok(ComposeState::Punging),
            "requested" => Ok(ComposeState::Requested),
            "signing_repo" => Ok(ComposeState::SigningRepo),
            "success" => Ok(ComposeState::Success),
            "syncing_repo" => Ok(ComposeState::SyncingRepo),
            "updateinfo" => Ok(ComposeState::UpdateInfo),
            _ => Err(InvalidValueError::new("ComposeStatus", value.to_owned())),
        }
    }
}

impl FromStr for ComposeState {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}

/// valid / known content types
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ContentType {
    // tag for container image updates
    #[serde(rename = "container")]
    Container,
    // tag for flatpak updates
    #[serde(rename = "flatpak")]
    Flatpak,
    // tag for module updates
    #[serde(rename = "module")]
    Module,
    // tag for traditional RPM package updates
    #[serde(rename = "rpm")]
    RPM,
}

impl ContentType {
    /// method for returning the [`FedoraRelease`] suffix corresponding to this [`ContentType`]
    pub const fn suffix(&self) -> &str {
        use ContentType::*;

        match self {
            RPM => "",
            Container => "C",
            Flatpak => "F",
            Module => "M",
        }
    }

    /// method for parsing [`FedoraRelease`] suffixes into a [`ContentType`]
    pub fn try_from_suffix(suffix: &str) -> Result<Self, InvalidValueError> {
        match suffix {
            "" => Ok(ContentType::RPM),
            "C" => Ok(ContentType::Container),
            "F" => Ok(ContentType::Flatpak),
            "M" => Ok(ContentType::Module),
            _ => Err(InvalidValueError::new(
                "ContentType",
                format!("Suffix '{suffix}' is not valid."),
            )),
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            ContentType::Container => "container",
            ContentType::Flatpak => "flatpak",
            ContentType::Module => "module",
            ContentType::RPM => "rpm",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for ContentType {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "container" => Ok(ContentType::Container),
            "flatpak" => Ok(ContentType::Flatpak),
            "module" => Ok(ContentType::Module),
            "rpm" => Ok(ContentType::RPM),
            _ => Err(InvalidValueError::new("ContentType", value.to_owned())),
        }
    }
}

impl FromStr for ContentType {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}

/// valid "karma" values that are associated for update comments and feedback
///
/// Only three values are valid: **-1** for positive feedback, **±0** for neutral (or unspecified)
/// feedback, and **-1** for negative feedback.
///
/// This type uses (de)serializaion support from [`serde_repr`] for converting these three numeric
/// values into the corresponding enum variants.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(i8)]
pub enum Karma {
    /// positive feedback
    Positive = 1,
    /// neutral / informational feedback (default)
    Neutral = 0,
    /// negative feedback
    Negative = -1,
}

impl Default for Karma {
    fn default() -> Self {
        Karma::Neutral
    }
}

impl Display for Karma {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Karma::Positive => String::from("+1"),
                Karma::Neutral => String::from("±0"),
                Karma::Negative => String::from("-1"),
            }
        )
    }
}

impl TryFrom<&str> for Karma {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "+1" | "1" => Ok(Karma::Positive),
            "0" | "±0" => Ok(Karma::Neutral),
            "-1" => Ok(Karma::Negative),
            _ => Err(InvalidValueError::new("Karma", value.to_owned())),
        }
    }
}

impl FromStr for Karma {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// valid / known package managers
///
/// Values of this type are used to print installation instructions for updates on the server.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PackageManager {
    #[serde(rename = "dnf")]
    DNF,
    #[serde(rename = "unspecified")]
    Unspecified,
    #[serde(rename = "yum")]
    YUM,
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            PackageManager::DNF => "dnf",
            PackageManager::Unspecified => "unspecified",
            PackageManager::YUM => "yum",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for PackageManager {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "dnf" => Ok(PackageManager::DNF),
            "yum" => Ok(PackageManager::YUM),
            _ => Err(InvalidValueError::new("PackageManager", value.to_owned())),
        }
    }
}

impl FromStr for PackageManager {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// valid `state` values for releases
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ReleaseState {
    /// release has been archived after it has reached its EOL
    #[serde(rename = "archived")]
    Archived,
    /// release is currently supported
    #[serde(rename = "current")]
    Current,
    /// release is disabled
    #[serde(rename = "disabled")]
    Disabled,
    /// release is frozen
    #[serde(rename = "frozen")]
    Frozen,
    /// release is in development
    #[serde(rename = "pending")]
    Pending,
}

impl Display for ReleaseState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            ReleaseState::Archived => "archived",
            ReleaseState::Current => "current",
            ReleaseState::Disabled => "disabled",
            ReleaseState::Frozen => "frozen",
            ReleaseState::Pending => "pending",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for ReleaseState {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "archived" => Ok(ReleaseState::Archived),
            "current" => Ok(ReleaseState::Current),
            "disabled" => Ok(ReleaseState::Disabled),
            "frozen" => Ok(ReleaseState::Frozen),
            "pending" => Ok(ReleaseState::Pending),
            _ => Err(InvalidValueError::new("ReleaseState", value.to_owned())),
        }
    }
}

impl FromStr for ReleaseState {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// valid `state` values for an update's gating tests
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TestGatingStatus {
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "greenwave_failed")]
    GreenwaveFailed,
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "passed")]
    Passed,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "waiting")]
    Waiting,
}

impl Display for TestGatingStatus {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            TestGatingStatus::Failed => "failed",
            TestGatingStatus::GreenwaveFailed => "greenwave_failed",
            TestGatingStatus::Ignored => "ignored",
            TestGatingStatus::Passed => "passed",
            TestGatingStatus::Queued => "queued",
            TestGatingStatus::Running => "running",
            TestGatingStatus::Waiting => "waiting",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for TestGatingStatus {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "failed" => Ok(TestGatingStatus::Failed),
            "greenwave_failed" => Ok(TestGatingStatus::GreenwaveFailed),
            "ignored" => Ok(TestGatingStatus::Ignored),
            "passed" => Ok(TestGatingStatus::Passed),
            "queued" => Ok(TestGatingStatus::Queued),
            "running" => Ok(TestGatingStatus::Running),
            "waiting" => Ok(TestGatingStatus::Waiting),
            _ => Err(InvalidValueError::new("TestGatingStatus", value.to_owned())),
        }
    }
}

impl FromStr for TestGatingStatus {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


// This enum represents the two possible ways to identify a fedora update:
// - internal, numerical ID (only for compatibility with old releases)
// - public, human-readable "alias" (`FEDORA-2019-1A2BB23E`)
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum UpdateID {
    // identified via numerical update ID
    ID(u32),
    // identified via update alias
    Alias(String),
}

impl Display for UpdateID {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            UpdateID::ID(number) => number.to_string(),
            UpdateID::Alias(string) => string.to_owned(),
        };

        write!(f, "{value}")
    }
}


/// valid `request` values for updates
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum UpdateRequest {
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

impl Display for UpdateRequest {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            UpdateRequest::Obsolete => "obsolete",
            UpdateRequest::Revoke => "revoke",
            UpdateRequest::Stable => "stable",
            UpdateRequest::Testing => "testing",
            UpdateRequest::Unpush => "unpush",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for UpdateRequest {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "obsolete" => Ok(UpdateRequest::Obsolete),
            "revoke" => Ok(UpdateRequest::Revoke),
            "stable" => Ok(UpdateRequest::Stable),
            "testing" => Ok(UpdateRequest::Testing),
            "unpush" => Ok(UpdateRequest::Unpush),
            _ => Err(InvalidValueError::new("UpdateRequest", value.to_owned())),
        }
    }
}

impl FromStr for UpdateRequest {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// valid `severity` values for updates
///
/// This field is required to not be `Unspecified` for updates with type [`UpdateType::Security`].
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

impl Default for UpdateSeverity {
    fn default() -> Self {
        UpdateSeverity::Unspecified
    }
}

impl Display for UpdateSeverity {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            UpdateSeverity::High => "high",
            UpdateSeverity::Low => "low",
            UpdateSeverity::Medium => "medium",
            UpdateSeverity::Unspecified => "unspecified",
            UpdateSeverity::Urgent => "urgent",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for UpdateSeverity {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "high" => Ok(UpdateSeverity::High),
            "low" => Ok(UpdateSeverity::Low),
            "medium" => Ok(UpdateSeverity::Medium),
            "unspecified" => Ok(UpdateSeverity::Unspecified),
            "urgent" => Ok(UpdateSeverity::Urgent),
            _ => Err(InvalidValueError::new("UpdateSeverity", value.to_owned())),
        }
    }
}

impl FromStr for UpdateSeverity {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// valid `status` values for updates
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum UpdateStatus {
    /// status of updates that have been obsoleted by another update
    #[serde(rename = "obsolete")]
    Obsolete,
    /// status of updates that are pending for either testing or stable
    #[serde(rename = "pending")]
    Pending,
    /// status of updates that are associated with an active side tag
    #[serde(rename = "side_tag_active")]
    SideTagActive,
    /// status of updates that are associated with an expired side tag
    #[serde(rename = "side_tag_expired")]
    SideTagExpired,
    /// status of updates that have been pushed to stable
    #[serde(rename = "stable")]
    Stable,
    /// status of updates that have been pushed to testing
    #[serde(rename = "testing")]
    Testing,
    /// status of updates that have been "unpushed" from testing
    #[serde(rename = "unpushed")]
    Unpushed,
}

impl Display for UpdateStatus {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            UpdateStatus::Obsolete => "obsolete",
            UpdateStatus::Pending => "pending",
            UpdateStatus::SideTagActive => "side_tag_active",
            UpdateStatus::SideTagExpired => "side_tag_expired",
            UpdateStatus::Stable => "stable",
            UpdateStatus::Testing => "testing",
            UpdateStatus::Unpushed => "unpushed",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for UpdateStatus {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "obsolete" => Ok(UpdateStatus::Obsolete),
            "pending" => Ok(UpdateStatus::Pending),
            "side_tag_active" => Ok(UpdateStatus::SideTagActive),
            "side_tag_expired" => Ok(UpdateStatus::SideTagExpired),
            "stable" => Ok(UpdateStatus::Stable),
            "testing" => Ok(UpdateStatus::Testing),
            "unpushed" => Ok(UpdateStatus::Unpushed),
            _ => Err(InvalidValueError::new("UpdateStatus", value.to_owned())),
        }
    }
}

impl FromStr for UpdateStatus {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// valid `suggestion` values for updates
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum UpdateSuggestion {
    /// recommendation for logging out after this update has been installed
    #[serde(rename = "logout")]
    Logout,
    /// recommendation for rebooting after this update has been installed
    #[serde(rename = "reboot")]
    Reboot,
    /// no recommendation (default)
    #[serde(rename = "unspecified")]
    Unspecified,
}

impl Default for UpdateSuggestion {
    fn default() -> Self {
        UpdateSuggestion::Unspecified
    }
}

impl Display for UpdateSuggestion {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            UpdateSuggestion::Logout => "logout",
            UpdateSuggestion::Reboot => "reboot",
            UpdateSuggestion::Unspecified => "unspecified",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for UpdateSuggestion {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "logout" => Ok(UpdateSuggestion::Logout),
            "reboot" => Ok(UpdateSuggestion::Reboot),
            "unspecified" => Ok(UpdateSuggestion::Unspecified),
            _ => Err(InvalidValueError::new("UpdateSuggestion", value.to_owned())),
        }
    }
}

impl FromStr for UpdateSuggestion {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// valid `type` values for updates
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum UpdateType {
    /// the update contains fixes for known bugs
    #[serde(rename = "bugfix")]
    BugFix,
    /// the update includes new features or other improvements
    #[serde(rename = "enhancement")]
    Enhancement,
    /// the update includes new packages
    #[serde(rename = "newpackage")]
    NewPackage,
    /// the update includes fixes for security problems
    #[serde(rename = "security")]
    Security,
    /// unspecified type (default)
    #[serde(rename = "unspecified")]
    Unspecified,
}

impl Default for UpdateType {
    fn default() -> Self {
        UpdateType::Unspecified
    }
}

impl Display for UpdateType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            UpdateType::BugFix => "bugfix",
            UpdateType::Enhancement => "enhancement",
            UpdateType::NewPackage => "newpackage",
            UpdateType::Security => "security",
            UpdateType::Unspecified => "unspecified",
        };

        write!(f, "{value}")
    }
}

impl TryFrom<&str> for UpdateType {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "bugfix" => Ok(UpdateType::BugFix),
            "enhancement" => Ok(UpdateType::Enhancement),
            "newpackage" => Ok(UpdateType::NewPackage),
            "security" => Ok(UpdateType::Security),
            "unspecified" => Ok(UpdateType::Unspecified),
            _ => Err(InvalidValueError::new("UpdateType", value.to_owned())),
        }
    }
}

impl FromStr for UpdateType {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}
