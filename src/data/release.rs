use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::InvalidValueError;

lazy_static! {
    pub(crate) static ref FEDORA_RELEASE_RE: Regex =
        Regex::new("^F(?P<number>[1-9][0-9]*)(?P<ctype>[CFM]?)$").expect("Failed to compile hard-coded regex!");
}

lazy_static! {
    pub(crate) static ref EPEL_RELEASE_RE: Regex =
        Regex::new("^EPEL-(?P<number>[1-9][0-9]*)(?P<ctype>[CFM]?)(?P<next>[N]?)$")
            .expect("Failed to compile hard-coded regex!");
}

lazy_static! {
    pub(crate) static ref EL_RELEASE_RE: Regex =
        Regex::new("^EL-(?P<number>[1-9][0-9]*)$").expect("Failed to compile hard-coded regex!");
}

fn release_parse_fedora(release: &str) -> Result<(u32, String), InvalidValueError> {
    let invalid = || InvalidValueError::new("FedoraRelease", release);

    let parsed = FEDORA_RELEASE_RE.captures(release).ok_or_else(invalid)?;
    let number: u32 = parsed
        .name("number")
        .ok_or_else(invalid)?
        .as_str()
        .parse::<u32>()
        .or(Err(invalid()))?;
    let ctype: String = parsed.name("ctype").ok_or_else(invalid)?.as_str().to_owned();

    Ok((number, ctype))
}

fn release_validate_fedora(release: &str) -> Result<FedoraRelease, InvalidValueError> {
    release_parse_fedora(release)?;
    Ok(FedoraRelease::from_str(release))
}

fn release_parse_epel(release: &str) -> Result<(u32, String, bool), InvalidValueError> {
    let invalid = || InvalidValueError::new("FedoraRelease", release);

    let parsed = EPEL_RELEASE_RE.captures(release).ok_or_else(invalid)?;
    let number: u32 = parsed
        .name("number")
        .ok_or_else(invalid)?
        .as_str()
        .parse::<u32>()
        .or(Err(invalid()))?;
    let ctype: String = parsed.name("ctype").ok_or_else(invalid)?.as_str().to_owned();
    let next: bool = parsed.name("next").ok_or_else(invalid)?.as_str() == "N";

    Ok((number, ctype, next))
}

fn release_validate_epel(release: &str) -> Result<FedoraRelease, InvalidValueError> {
    release_parse_epel(release)?;
    Ok(FedoraRelease::from_str(release))
}

fn release_parse_el(release: &str) -> Result<u32, InvalidValueError> {
    let invalid = || InvalidValueError::new("FedoraRelease", release);

    let parsed = EL_RELEASE_RE.captures(release).ok_or_else(invalid)?;
    let number: u32 = parsed
        .name("number")
        .ok_or_else(invalid)?
        .as_str()
        .parse::<u32>()
        .or(Err(invalid()))?;

    Ok(number)
}

fn release_validate_el(release: &str) -> Result<FedoraRelease, InvalidValueError> {
    release_parse_el(release)?;
    Ok(FedoraRelease::from_str(release))
}

/*
enum FedoraReleaseType {
    Meta { value: Cow<'static, str> },
    ELN,
    Fedora { number: u32, ctype: ContentType },
    Epel { number: u32, ctype: ContentType },
    El { number: u32 },
}
*/

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
pub struct FedoraRelease {
    release: Cow<'static, str>,
}

impl FedoraRelease {
    pub const CURRENT: Self = Self::from_static_str("__current__");
    pub const PENDING: Self = Self::from_static_str("__pending__");
    pub const ARCHIVED: Self = Self::from_static_str("__archived__");

    // internal method for constructing instances in const contexts
    const fn from_static_str(string: &'static str) -> Self {
        FedoraRelease {
            release: Cow::Borrowed(string),
        }
    }

    // internal method for constructing instances from verified contents
    fn from_str(string: &str) -> Self {
        FedoraRelease {
            release: Cow::Owned(String::from(string)),
        }
    }
}

impl Display for FedoraRelease {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.release)
    }
}

impl TryFrom<&str> for FedoraRelease {
    type Error = InvalidValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "" => Err(InvalidValueError::new("FedoraRelease", "(empty string)")),
            "ELN" => Ok(FedoraRelease::from_str("ELN")),
            f if f.starts_with('F') => release_validate_fedora(f),
            epel if epel.starts_with("EPEL") => release_validate_epel(epel),
            el if el.starts_with("EL") => release_validate_el(el),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_eln() {
        FedoraRelease::try_from("ELN").unwrap();
    }

    #[test]
    fn parse_fedora() {
        #[rustfmt::skip]
        let fixtures = [
            ("F36", (36, "")), ("F36C", (36, "C")), ("F36F", (36, "F")), ("F36M", (36, "M")),
            ("F35", (35, "")), ("F35C", (35, "C")), ("F35F", (35, "F")), ("F35M", (35, "M")),
            ("F34", (34, "")), ("F34C", (34, "C")), ("F34F", (34, "F")), ("F34M", (34, "M")),
            ("F33", (33, "")), ("F33C", (33, "C")), ("F33F", (33, "F")), ("F33M", (33, "M")),
            ("F32", (32, "")), ("F32C", (32, "C")), ("F32F", (32, "F")), ("F32M", (32, "M")),
            ("F31", (31, "")), ("F31C", (31, "C")), ("F31F", (31, "F")), ("F31M", (31, "M")),
            ("F30", (30, "")), ("F30C", (30, "C")), ("F30F", (30, "F")), ("F30M", (30, "M")),
            ("F29", (29, "")), ("F29C", (29, "C")), ("F29F", (29, "F")), ("F29M", (29, "M")),
            ("F28", (28, "")), ("F28C", (28, "C")), ("F28M", (28, "M")),
            ("F27", (27, "")), ("F27M", (27, "M")),
            ("F26", (26, "")),
            ("F25", (25, "")),
            ("F24", (24, "")),
            ("F23", (23, "")),
            ("F22", (22, "")),
            ("F21", (21, "")),
        ];

        for (value, expected) in fixtures {
            let parsed = release_parse_fedora(value).unwrap();

            assert_eq!(parsed.0, expected.0);
            assert_eq!(parsed.1, expected.1);

            FedoraRelease::try_from(value).unwrap();
        }
    }

    #[test]
    fn parse_epel() {
        #[rustfmt::skip]
        let fixtures = [
            ("EPEL-9", (9, "", false)), ("EPEL-9N", (9, "", true)),
            ("EPEL-8", (8, "", false)), ("EPEL-8M", (8, "M", false)), ("EPEL-8N", (8, "", true)),
            ("EPEL-7", (7, "", false)),
        ];

        for (value, expected) in fixtures {
            let parsed = release_parse_epel(value).unwrap();

            assert_eq!(parsed.0, expected.0);
            assert_eq!(parsed.1, expected.1);
            assert_eq!(parsed.2, expected.2);

            FedoraRelease::try_from(value).unwrap();
        }
    }

    #[test]
    fn parse_el() {
        #[rustfmt::skip]
        let fixtures = [("EL-6", 6), ("EL-5", 5)];

        for (value, expected) in fixtures {
            let parsed = release_parse_el(value).unwrap();

            assert_eq!(parsed, expected);

            FedoraRelease::try_from(value).unwrap();
        }
    }
}
