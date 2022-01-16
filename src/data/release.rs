use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use super::enums::{ContentType, InvalidValueError};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
pub struct FedoraRelease {
    release: Cow<'static, str>,
}

impl FedoraRelease {
    pub const CURRENT: Self = Self::from_str_unchecked("__current__");
    pub const PENDING: Self = Self::from_str_unchecked("__pending__");
    pub const ARCHIVED: Self = Self::from_str_unchecked("__archived__");

    const fn from_str_unchecked(string: &'static str) -> Self {
        FedoraRelease {
            release: Cow::Borrowed(string),
        }
    }

    fn try_from_fedora_parts(number: u32, ctype: ContentType) -> Result<Self, InvalidValueError> {
        // Fedora releases older than 21 are not supported by bodhi
        if number < 21 {
            Err(InvalidValueError::new("FedoraRelease", "release number < 21"))
        } else {
            Ok(FedoraRelease {
                release: Cow::Owned(format!("F{}{}", number, ctype.suffix())),
            })
        }
    }
}

impl Display for FedoraRelease {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.release)
    }
}

/*
// This enum represents a fedora release.
// When updating this enum for new releases:
// - also add them in the impls below,
// - add them to the idem_fedora_release test,
// - add integration tests and data for them.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum OldFedoraRelease {
    F36,
    F36C,
    F36F,
    F36M,
    F35,
    F35C,
    F35F,
    F35M,
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
    #[serde(rename = "EPEL-9")]
    EPEL9,
    #[serde(rename = "EPEL-9N")]
    EPEL9N,
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

impl TryFrom<&str> for FedoraRelease {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "F36" => Ok(FedoraRelease::F36),
            "F36C" => Ok(FedoraRelease::F36C),
            "F36F" => Ok(FedoraRelease::F36F),
            "F36M" => Ok(FedoraRelease::F36M),
            "F35" => Ok(FedoraRelease::F35),
            "F35C" => Ok(FedoraRelease::F35C),
            "F35F" => Ok(FedoraRelease::F35F),
            "F35M" => Ok(FedoraRelease::F35M),
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
            "EPEL-9" => Ok(FedoraRelease::EPEL9),
            "EPEL-9N" => Ok(FedoraRelease::EPEL9N),
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
*/
