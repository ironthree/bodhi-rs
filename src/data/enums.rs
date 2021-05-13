use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// This error is returned when a string fails to be parsed into an enumerated value.
#[derive(Debug)]
pub struct InvalidValueError {
    /// This field contains the name of the enum.
    pub name: &'static str,
    /// This field contains the invalid value.
    pub value: String,
}

impl InvalidValueError {
    fn new(name: &'static str, value: &str) -> Self {
        InvalidValueError {
            name,
            value: value.to_owned(),
        }
    }
}

impl Display for InvalidValueError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Invalid value for {}: {}", self.name, self.value)
    }
}

impl Error for InvalidValueError {}

/// This enum represents the possible request values for composes.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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

        write!(f, "{}", value)
    }
}

impl TryFrom<&str> for ComposeRequest {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "stable" => Ok(ComposeRequest::Stable),
            "testing" => Ok(ComposeRequest::Testing),
            _ => Err(InvalidValueError::new("ComposeRequest", value)),
        }
    }
}

impl FromStr for ComposeRequest {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents the possible status values for composes.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum ComposeStatus {
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

impl Display for ComposeStatus {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            ComposeStatus::Cleaning => "cleaning",
            ComposeStatus::Failed => "failed",
            ComposeStatus::Initializing => "initializing",
            ComposeStatus::Notifying => "notifying",
            ComposeStatus::Pending => "pending",
            ComposeStatus::Punging => "punging",
            ComposeStatus::Requested => "requested",
            ComposeStatus::SigningRepo => "signing_repo",
            ComposeStatus::Success => "success",
            ComposeStatus::SyncingRepo => "syncing_repo",
            ComposeStatus::UpdateInfo => "updateinfo",
        };

        write!(f, "{}", value)
    }
}

impl TryFrom<&str> for ComposeStatus {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "cleaning" => Ok(ComposeStatus::Cleaning),
            "failed" => Ok(ComposeStatus::Failed),
            "initializing" => Ok(ComposeStatus::Initializing),
            "notifying" => Ok(ComposeStatus::Notifying),
            "pending" => Ok(ComposeStatus::Pending),
            "punging" => Ok(ComposeStatus::Punging),
            "requested" => Ok(ComposeStatus::Requested),
            "signing_repo" => Ok(ComposeStatus::SigningRepo),
            "success" => Ok(ComposeStatus::Success),
            "syncing_repo" => Ok(ComposeStatus::SyncingRepo),
            "updateinfo" => Ok(ComposeStatus::UpdateInfo),
            _ => Err(InvalidValueError::new("ComposeStatus", value)),
        }
    }
}

impl FromStr for ComposeStatus {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}

/// This enum represents the type of a bodhi update, of a package, and of builds.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum ContentType {
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

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            ContentType::Container => "container",
            ContentType::Flatpak => "flatpak",
            ContentType::Module => "module",
            ContentType::RPM => "rpm",
        };

        write!(f, "{}", value)
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
            _ => Err(InvalidValueError::new("ContentType", value)),
        }
    }
}

impl FromStr for ContentType {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents a fedora release.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
// When updating this enum for new releases:
// - also add them in the impls below,
// - add them to the idem_fedora_release test,
// - add integration tests and data for them.
pub enum FedoraRelease {
    #[serde(rename = "__current__")]
    Current,
    #[serde(rename = "__pending__")]
    Pending,
    #[serde(rename = "__archived__")]
    Archived,
    F35,
    F35C,
    F34,
    F34C,
    F34F,
    F34M,
    F33,
    F33C,
    F33F,
    F33M,
    F32,
    F32C,
    F32F,
    F32M,
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
    #[serde(rename = "EPEL-8N")]
    EPEL8N,
    #[serde(rename = "EPEL-7")]
    EPEL7,
    #[serde(rename = "EL-6")]
    EL6,
    #[serde(rename = "EL-5")]
    EL5,
    ELN,
}

impl Display for FedoraRelease {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            FedoraRelease::Current => "__current__",
            FedoraRelease::Pending => "__pending__",
            FedoraRelease::Archived => "__archived__",
            FedoraRelease::F35 => "F35",
            FedoraRelease::F35C => "F35C",
            FedoraRelease::F34 => "F34",
            FedoraRelease::F34C => "F34C",
            FedoraRelease::F34F => "F34F",
            FedoraRelease::F34M => "F34M",
            FedoraRelease::F33 => "F33",
            FedoraRelease::F33C => "F33C",
            FedoraRelease::F33F => "F33F",
            FedoraRelease::F33M => "F33M",
            FedoraRelease::F32 => "F32",
            FedoraRelease::F32C => "F32C",
            FedoraRelease::F32F => "F32F",
            FedoraRelease::F32M => "F32M",
            FedoraRelease::F31 => "F31",
            FedoraRelease::F31C => "F31C",
            FedoraRelease::F31F => "F31F",
            FedoraRelease::F31M => "F31M",
            FedoraRelease::F30 => "F30",
            FedoraRelease::F30C => "F30C",
            FedoraRelease::F30F => "F30F",
            FedoraRelease::F30M => "F30M",
            FedoraRelease::F29 => "F29",
            FedoraRelease::F29C => "F29C",
            FedoraRelease::F29F => "F29F",
            FedoraRelease::F29M => "F29M",
            FedoraRelease::F28 => "F28",
            FedoraRelease::F28C => "F28C",
            FedoraRelease::F28M => "F28M",
            FedoraRelease::F27 => "F27",
            FedoraRelease::F27M => "F27M",
            FedoraRelease::F26 => "F26",
            FedoraRelease::F25 => "F25",
            FedoraRelease::F24 => "F24",
            FedoraRelease::F23 => "F23",
            FedoraRelease::F22 => "F22",
            FedoraRelease::F21 => "F21",
            FedoraRelease::EPEL8 => "EPEL-8",
            FedoraRelease::EPEL8M => "EPEL-8M",
            FedoraRelease::EPEL8N => "EPEL-8N",
            FedoraRelease::EPEL7 => "EPEL-7",
            FedoraRelease::EL6 => "EL-6",
            FedoraRelease::EL5 => "EL-5",
            FedoraRelease::ELN => "ELN",
        };

        write!(f, "{}", value)
    }
}

impl TryFrom<&str> for FedoraRelease {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "F35" => Ok(FedoraRelease::F35),
            "F35C" => Ok(FedoraRelease::F35C),
            "F34" => Ok(FedoraRelease::F34),
            "F34C" => Ok(FedoraRelease::F34C),
            "F34F" => Ok(FedoraRelease::F34F),
            "F34M" => Ok(FedoraRelease::F34M),
            "F33" => Ok(FedoraRelease::F33),
            "F33C" => Ok(FedoraRelease::F33C),
            "F33F" => Ok(FedoraRelease::F33F),
            "F33M" => Ok(FedoraRelease::F33M),
            "F32" => Ok(FedoraRelease::F32),
            "F32C" => Ok(FedoraRelease::F32C),
            "F32F" => Ok(FedoraRelease::F32F),
            "F32M" => Ok(FedoraRelease::F32M),
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
            "EPEL-8N" => Ok(FedoraRelease::EPEL8N),
            "EPEL-7" => Ok(FedoraRelease::EPEL7),
            "EL-6" => Ok(FedoraRelease::EL6),
            "EL-5" => Ok(FedoraRelease::EL5),
            "ELN" => Ok(FedoraRelease::ELN),
            _ => Err(InvalidValueError::new("FedoraRelease", value)),
        }
    }
}

impl FromStr for FedoraRelease {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents a "Karma" value, which is either a positive (+1), neutral (±0), or negative
/// (-1) feedback for an update, and is associated with a [`Comment`](struct.Comment.html), and
/// possibly also a [`TestCaseFeedback`](struct.TestCase.html) or a
/// [`BugFeedback`](struct.BugFeedback.html).
#[derive(Clone, Copy, Debug, Deserialize_repr, PartialEq, Serialize_repr)]
#[repr(i8)]
pub enum Karma {
    /// positive feedback
    Positive = 1,
    /// neutral / informational feedback
    Neutral = 0,
    /// negative feedback
    Negative = -1,
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
            _ => Err(InvalidValueError::new("Karma", value)),
        }
    }
}

impl FromStr for Karma {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents the name of the package manager that's in use on a release.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, PartialEq, Serialize)]
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

        write!(f, "{}", value)
    }
}

impl TryFrom<&str> for PackageManager {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "dnf" => Ok(PackageManager::DNF),
            "yum" => Ok(PackageManager::YUM),
            _ => Err(InvalidValueError::new("PackageManager", value)),
        }
    }
}

impl FromStr for PackageManager {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents the state of a release.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
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

        write!(f, "{}", value)
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
            _ => Err(InvalidValueError::new("ReleaseState", value)),
        }
    }
}

impl FromStr for ReleaseState {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents the test gating status from `greenwave`.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
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

        write!(f, "{}", value)
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
            _ => Err(InvalidValueError::new("TestGatingStatus", value)),
        }
    }
}

impl FromStr for TestGatingStatus {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents the two possible ways to identify a fedora update:
/// - internal, numerical ID
/// - public, human-readable "alias" (`FEDORA-2019-1A2BB23E`)
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum UpdateID {
    /// identified via numerical update ID
    ID(u32),
    /// identified via update alias
    Alias(String),
}

impl Display for UpdateID {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            UpdateID::ID(number) => number.to_string(),
            UpdateID::Alias(string) => string.to_owned(),
        };

        write!(f, "{}", value)
    }
}


/// This enum represents a requested state change of an update.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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

        write!(f, "{}", value)
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
            _ => Err(InvalidValueError::new("UpdateRequest", value)),
        }
    }
}

impl FromStr for UpdateRequest {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents the associated severity of a bodhi update. This field is required to not be
/// unspecified for updates with [`UpdateType::Security`](enum.UpdateType.html).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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

impl Display for UpdateSeverity {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            UpdateSeverity::High => "high",
            UpdateSeverity::Low => "low",
            UpdateSeverity::Medium => "medium",
            UpdateSeverity::Unspecified => "unspecified",
            UpdateSeverity::Urgent => "urgent",
        };

        write!(f, "{}", value)
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
            _ => Err(InvalidValueError::new("UpdateSeverity", value)),
        }
    }
}

impl FromStr for UpdateSeverity {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents the current state of a bodhi update.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum UpdateStatus {
    /// tag for updates that have been obsoleted by another update
    #[serde(rename = "obsolete")]
    Obsolete,
    /// tag for updates that are pending for either testing or stable
    #[serde(rename = "pending")]
    Pending,
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

        write!(f, "{}", value)
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
            _ => Err(InvalidValueError::new("UpdateStatus", value)),
        }
    }
}

impl FromStr for UpdateStatus {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents the associated suggested action for a bodhi update.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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

impl Display for UpdateSuggestion {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            UpdateSuggestion::Logout => "logout",
            UpdateSuggestion::Reboot => "reboot",
            UpdateSuggestion::Unspecified => "unspecified",
        };

        write!(f, "{}", value)
    }
}

impl TryFrom<&str> for UpdateSuggestion {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "logout" => Ok(UpdateSuggestion::Logout),
            "reboot" => Ok(UpdateSuggestion::Reboot),
            "unspecified" => Ok(UpdateSuggestion::Unspecified),
            _ => Err(InvalidValueError::new("UpdateSuggestion", value)),
        }
    }
}

impl FromStr for UpdateSuggestion {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}


/// This enum represents the type of a bodhi update.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum UpdateType {
    #[serde(rename = "bugfix")]
    BugFix,
    #[serde(rename = "enhancement")]
    Enhancement,
    #[serde(rename = "newpackage")]
    NewPackage,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "unspecified")]
    Unspecified,
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

        write!(f, "{}", value)
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
            _ => Err(InvalidValueError::new("UpdateType", value)),
        }
    }
}

impl FromStr for UpdateType {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}
