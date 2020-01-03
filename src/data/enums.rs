use std::cmp::PartialEq;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

const INTERNAL_ERROR: &str = "This should never happen, since all hard-coded values for enums are sane.";

/// This enum represents the two possible values of compose checkpoints:
/// - the empty object (`{}`), which does not correctly deserialize into an empty `HashMap`, and
/// - a map of Strings to booleans.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Checkpoints {
    /// This value represents an empty map (`{}`).
    None(String),
    /// This value represents a non-empty map of checkpoints.
    Map(HashMap<String, bool>),
}


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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
    }
}


/// This enum represents a fedora release.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
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

impl Display for FedoraRelease {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = match self {
            FedoraRelease::F32 => "F32",
            FedoraRelease::F32C => "F32C",
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
            FedoraRelease::EPEL7 => "EPEL-7",
            FedoraRelease::EL6 => "EL-6",
            FedoraRelease::EL5 => "EL-5",
        };

        write!(f, "{}", value)
    }
}

impl TryFrom<&str> for FedoraRelease {
    type Error = &'static str;

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
            _ => Err("Unrecognised release."),
        }
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


/// This enum represents the name of the package manager that's in use on a release.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Serialize)]
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
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
        write!(f, "{}", serde_json::to_string(self).expect(INTERNAL_ERROR))
    }
}
