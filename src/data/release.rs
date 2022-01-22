use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{ContentType, InvalidValueError};

mod fedora {
    use lazy_static::lazy_static;
    use regex::Regex;

    use super::{ContentType, FedoraRelease, InvalidValueError};

    lazy_static! {
        pub static ref RELEASE_RE: Regex =
            Regex::new("^F(?P<number>[1-9][0-9]*)(?P<ctype>[CFM]?)$").expect("Failed to compile hard-coded regex!");
    }

    pub fn release_parse(release: &str) -> Result<(u32, String), InvalidValueError> {
        let invalid = || InvalidValueError::new("FedoraRelease", release.to_owned());

        let parsed = RELEASE_RE.captures(release).ok_or_else(invalid)?;
        let number: u32 = parsed
            .name("number")
            .ok_or_else(invalid)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| invalid())?;
        let ctype: String = parsed.name("ctype").ok_or_else(invalid)?.as_str().to_owned();

        Ok((number, ctype))
    }

    pub const MIN_RELEASE: u32 = 21;
    pub const MIN_CONTAINER_RELEASE: u32 = 28;
    pub const MIN_FLATPAK_RELEASE: u32 = 29;
    pub const MIN_MODULE_RELEASE: u32 = 27;

    pub fn is_valid_release(number: u32, ctype: ContentType) -> bool {
        use ContentType::*;

        match ctype {
            RPM => number >= MIN_RELEASE,
            Container => number >= MIN_CONTAINER_RELEASE,
            Flatpak => number >= MIN_FLATPAK_RELEASE,
            Module => number >= MIN_MODULE_RELEASE,
        }
    }

    pub fn release_validate(release: &str) -> Result<FedoraRelease, InvalidValueError> {
        let (num, ctype) = release_parse(release)?;

        if !is_valid_release(num, ContentType::try_from_suffix(&ctype)?) {
            return Err(InvalidValueError::new("FedoraRelease", release.to_string()));
        }

        Ok(FedoraRelease::from_str(release))
    }
}

mod epel {
    use lazy_static::lazy_static;
    use regex::Regex;

    use super::{ContentType, FedoraRelease, InvalidValueError};

    lazy_static! {
        pub static ref RELEASE_RE: Regex = Regex::new("^EPEL-(?P<number>[1-9][0-9]*)(?P<ctype>[CFM]?)(?P<next>[N]?)$")
            .expect("Failed to compile hard-coded regex!");
    }

    pub fn release_parse(release: &str) -> Result<(u32, String, bool), InvalidValueError> {
        let invalid = || InvalidValueError::new("FedoraRelease", release.to_owned());

        let parsed = RELEASE_RE.captures(release).ok_or_else(invalid)?;
        let number: u32 = parsed
            .name("number")
            .ok_or_else(invalid)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| invalid())?;
        let ctype: String = parsed.name("ctype").ok_or_else(invalid)?.as_str().to_owned();
        let next: bool = parsed.name("next").ok_or_else(invalid)?.as_str() == "N";

        Ok((number, ctype, next))
    }

    pub const MIN_RELEASE: u32 = 7;
    pub const MIN_MODULE_RELEASE: u32 = 8;
    pub const MIN_NEXT_RELEASE: u32 = 8;

    pub fn is_valid_release(number: u32, ctype: ContentType, next: bool) -> bool {
        use ContentType::*;

        let valid_type = match ctype {
            RPM => number >= MIN_RELEASE,
            Container => false,
            Flatpak => false,
            Module => number >= MIN_MODULE_RELEASE,
        };

        let valid_next = match next {
            false => number >= MIN_RELEASE,
            true => number >= MIN_NEXT_RELEASE,
        };

        let valid_combo = (ctype == RPM) || !next;

        valid_type && valid_next && valid_combo
    }

    pub fn release_validate(release: &str) -> Result<FedoraRelease, InvalidValueError> {
        let (num, ctype, next) = release_parse(release)?;

        if !(is_valid_release(num, ContentType::try_from_suffix(&ctype)?, next)) {
            return Err(InvalidValueError::new("FedoraRelease", release.to_string()));
        }

        Ok(FedoraRelease::from_str(release))
    }
}

mod el {
    use lazy_static::lazy_static;
    use regex::Regex;

    use super::{FedoraRelease, InvalidValueError};

    lazy_static! {
        pub static ref RELEASE_RE: Regex =
            Regex::new("^EL-(?P<number>[1-9][0-9]*)$").expect("Failed to compile hard-coded regex!");
    }

    pub fn release_parse(release: &str) -> Result<u32, InvalidValueError> {
        let invalid = || InvalidValueError::new("FedoraRelease", release.to_owned());

        let parsed = RELEASE_RE.captures(release).ok_or_else(invalid)?;
        let number: u32 = parsed
            .name("number")
            .ok_or_else(invalid)?
            .as_str()
            .parse::<u32>()
            .map_err(|_| invalid())?;

        Ok(number)
    }

    pub const MIN_RELEASE: u32 = 5;
    pub const MAX_RELEASE: u32 = 6;

    pub fn is_valid_release(number: u32) -> bool {
        (MIN_RELEASE..=MAX_RELEASE).contains(&number)
    }

    pub fn release_validate(release: &str) -> Result<FedoraRelease, InvalidValueError> {
        let num = release_parse(release)?;

        if !(is_valid_release(num)) {
            return Err(InvalidValueError::new("FedoraRelease", release.to_string()));
        }

        Ok(FedoraRelease::from_str(release))
    }
}


/// newtype wrapper around strings that represents a valid Fedora or EPEL release identifier
///
/// [`FedoraRelease`] is implemented as a newtype wrapper around strings, but all public methods of
/// constructing values ensure only instances containing valid release identifiers can be built.
///
/// The regular expressions that are used to validate and parse strings into valid [`FedoraRelease`]
/// values are defined in a way that should make future adjustments for new releases unnecessary.
/// For example, arbitrarily large numbers are supported. The only hard-coded parameters are the
/// known suffixes:
///
/// - no suffix: [`ContentType::RPM`]
/// - suffix `C`: [`ContentType::Container`]
/// - suffix `F`: [`ContentType::Flatpak`]
/// - suffix `M`: [`ContentType::Module`]
/// - suffix `N`: EPEL-next
///
/// Additionally, there are predefined [`FedoraRelease`] constants for nonvariable releases, and for
/// special values that are accepted by bodhi queries:
///
/// - [`FedoraRelease::ELN`]
/// - [`FedoraRelease::CURRENT`]
/// - [`FedoraRelease::PENDING`]
/// - [`FedoraRelease::ARCHIVED`]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
pub struct FedoraRelease {
    release: Cow<'static, str>,
}

impl FedoraRelease {
    /// constant that refers to all releases that are currently supported
    pub const CURRENT: Self = Self::from_static_str("__current__");
    /// constant that refers to all releases that are currently in development
    pub const PENDING: Self = Self::from_static_str("__pending__");
    /// constant that refers to all releases which have been archived after their end-of-life (EOL)
    pub const ARCHIVED: Self = Self::from_static_str("__archived__");

    /// constant that refers to the static "ELN" ("Enterprise Linux Next") release
    pub const ELN: Self = Self::from_static_str("ELN");

    // internal method for constructing instances in const contexts
    const fn from_static_str(string: &'static str) -> Self {
        FedoraRelease {
            release: Cow::Borrowed(string),
        }
    }

    // internal method for constructing instances from verified borrowed strings
    fn from_str(string: &str) -> Self {
        FedoraRelease {
            release: Cow::Owned(String::from(string)),
        }
    }

    /// construct and validate a Fedora [`FedoraRelease`] value from its parts
    ///
    /// The validation process takes various constraints into account:
    ///
    /// - release number of the first Fedora release known to bodhi (Fedora 21)
    /// - release number of the first Fedora release that supported Containers (Fedora 28)
    /// - release number of the first Fedora release that supported Modules (Fedora 27)
    /// - release number of the first Fedora release that supported Flatpaks (Fedora 29)
    ///
    /// However, since no information about the future is available, no maximum supported
    /// Fedora release is checked against.
    pub fn fedora(number: u32, ctype: ContentType) -> Result<Self, InvalidValueError> {
        let string = format!("F{}{}", number, ctype.suffix());
        string.parse()
    }

    /// construct and validate a EPEL [`FedoraRelease`] value from its parts
    ///
    /// The validation process takes various constraints into account:
    ///
    /// - release number of the first EPEL releases known to bodhi (EL-5, EPEL-7)
    /// - release numbers for which the identifier prefix is `EL-` (5, 6)
    /// - release numbers for which the identifier prefix is `EPEL-` (7+)
    /// - which content types are valid for EPEL releases (RPMs and Modules)
    /// - release number of the first EPEL release that supported Modules (8)
    /// - release number of the first EPEL release with a `-next` branch (8)
    ///
    /// However, no maximum release numbers are checked against during validation, due to lack of
    /// information about future events.
    pub fn epel(number: u32, ctype: ContentType, next: bool) -> Result<Self, InvalidValueError> {
        let prefix = if number < 7 { "EL" } else { "EPEL" };
        let suffix = if next { "N" } else { "" };
        let string = format!("{}-{}{}{}", prefix, number, ctype.suffix(), suffix);
        string.parse()
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
            "" => Err(InvalidValueError::new("FedoraRelease", String::from("(empty string)"))),
            "ELN" => Ok(FedoraRelease::from_str("ELN")),
            f if f.starts_with('F') => fedora::release_validate(f),
            epel if epel.starts_with("EPEL") => epel::release_validate(epel),
            el if el.starts_with("EL") => el::release_validate(el),
            _ => Err(InvalidValueError::new("FedoraRelease", value.to_owned())),
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
#[allow(clippy::unwrap_used)]
mod tests {
    use quickcheck::Gen;
    use quickcheck_macros::quickcheck;

    use super::*;

    // hacky implementation of the Arbitrary trait for ContentType
    // this only works because the enum has exactly four variants
    impl quickcheck::Arbitrary for ContentType {
        fn arbitrary(g: &mut Gen) -> Self {
            match (bool::arbitrary(g), bool::arbitrary(g)) {
                (true, true) => ContentType::RPM,
                (true, false) => ContentType::Container,
                (false, true) => ContentType::Flatpak,
                (false, false) => ContentType::Module,
            }
        }
    }

    #[test]
    fn parse_print_all() {
        #[rustfmt::skip]
        let known =[
            "F36", "F36C",
            "F35", "F35C", "F35F", "F35M",
            "F34", "F34C", "F34F", "F34M",
            "F33", "F33C", "F33F", "F33M",
            "F32", "F32C", "F32F", "F32M",
            "F31", "F31C", "F31F", "F31M",
            "F30", "F30C", "F30F", "F30M",
            "F29", "F29C", "F29F", "F29M",
            "F28", "F28C", "F28M",
            "F27", "F27M",
            "F26",
            "F25",
            "F24",
            "F23",
            "F22",
            "F21",
            "EPEL-9", "EPEL-9N",
            "EPEL-8", "EPEL-8M", "EPEL-8N",
            "EPEL-7",
            "EL-6",
            "EL-5",
            "ELN",
        ];

        // check if the parsing function can parse all known values
        // (on its own, this does not check correctness)
        // and can return the same string value again
        for value in known {
            let parsed = FedoraRelease::try_from(value).unwrap();
            assert_eq!(value, parsed.to_string());
        }
    }

    #[test]
    fn parse_eln() {
        let eln = FedoraRelease::try_from("ELN").unwrap();

        // assert that a manually constructed value is equal to the constant
        assert_eq!(FedoraRelease::ELN.to_string(), "ELN");
        assert_eq!(FedoraRelease::ELN, eln);
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
            // check if the parsing function can parse all known values
            let parsed = fedora::release_parse(value).unwrap();

            // check if the parser returns the correct values
            assert_eq!(parsed.0, expected.0);
            assert_eq!(parsed.1, expected.1);

            // check of the constructor accepts all known values
            let release = FedoraRelease::try_from(value).unwrap();
            assert_eq!(release.to_string(), value);
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
            // check if the parsing function can parse all known values
            let parsed = epel::release_parse(value).unwrap();

            // check if the parser returns the correct values
            assert_eq!(parsed.0, expected.0);
            assert_eq!(parsed.1, expected.1);
            assert_eq!(parsed.2, expected.2);

            // check of the constructor accepts all known values
            let release = FedoraRelease::try_from(value).unwrap();
            assert_eq!(release.to_string(), value);
        }
    }

    #[test]
    fn parse_el() {
        #[rustfmt::skip]
        let fixtures = [("EL-6", 6), ("EL-5", 5)];

        for (value, expected) in fixtures {
            // check if the parsing function can parse all known values
            let parsed = el::release_parse(value).unwrap();

            // check if the parser returns the correct values
            assert_eq!(parsed, expected);

            // check of the constructor accepts all known values
            let release = FedoraRelease::try_from(value).unwrap();
            assert_eq!(release.to_string(), value);
        }
    }

    #[test]
    fn parse_invalid() {
        #[rustfmt::skip]
        let values = [
            "F20",      // too old
            "F21C",     // no container support
            "F22F",     // no flatpak support
            "F23M",     // no module support
            "EPEL-2",   // too old
            "EPEL-3N",  // no next support
            "EPEL-7M",  // no module support
            "EPEL-8CN", // invalid combo
            "EPEL-8FN", // invalid combo
            "EPEL-8MN", // invalid combo
            "EPEL-9CN", // invalid combo
            "EPEL-9FN", // invalid combo
            "EPEL-9MN", // invalid combo
            "EL-10",    // too new
        ];

        for value in values {
            value.parse::<FedoraRelease>().unwrap_err();
        }
    }

    #[quickcheck]
    fn check_fedora(number: u32) -> bool {
        if number < fedora::MIN_RELEASE {
            return true;
        }

        let built = FedoraRelease::fedora(number, ContentType::RPM).unwrap().to_string();
        let (num, ctype) = fedora::release_parse(&built).unwrap();
        number == num && ctype.is_empty()
    }

    #[quickcheck]
    fn check_fedora_container(number: u32) -> bool {
        if number < fedora::MIN_RELEASE {
            return true;
        }

        let built = FedoraRelease::fedora(number, ContentType::Container)
            .unwrap()
            .to_string();
        let (num, ctype) = fedora::release_parse(&built).unwrap();
        number == num && ctype == "C"
    }

    #[quickcheck]
    fn check_fedora_flatpak(number: u32) -> bool {
        if number < fedora::MIN_RELEASE {
            return true;
        }

        let built = FedoraRelease::fedora(number, ContentType::Flatpak).unwrap().to_string();
        let (num, ctype) = fedora::release_parse(&built).unwrap();
        number == num && ctype == "F"
    }

    #[quickcheck]
    fn check_fedora_module(number: u32) -> bool {
        if number < fedora::MIN_RELEASE {
            return true;
        }

        let built = FedoraRelease::fedora(number, ContentType::Module).unwrap().to_string();
        let (num, ctype) = fedora::release_parse(&built).unwrap();
        number == num && ctype == "M"
    }

    #[quickcheck]
    fn check_epel(number: u32) -> bool {
        if number < epel::MIN_RELEASE {
            return true;
        }

        let built = FedoraRelease::epel(number, ContentType::RPM, false)
            .unwrap()
            .to_string();
        let num = if number < 7 {
            el::release_parse(&built).unwrap()
        } else {
            let (num, ctype, next) = epel::release_parse(&built).unwrap();
            assert!(ctype.is_empty());
            assert!(!next);
            num
        };
        number == num
    }

    #[quickcheck]
    fn check_epel_modules(number: u32) -> bool {
        if number < epel::MIN_MODULE_RELEASE {
            return true;
        }

        let built = FedoraRelease::epel(number, ContentType::Module, false)
            .unwrap()
            .to_string();
        let (num, ctype, next) = epel::release_parse(&built).unwrap();
        number == num && ctype == "M" && !next
    }

    #[quickcheck]
    fn check_epel_next(number: u32) -> bool {
        if number < epel::MIN_NEXT_RELEASE {
            return true;
        }

        let built = FedoraRelease::epel(number, ContentType::RPM, true).unwrap().to_string();
        let (num, ctype, next) = epel::release_parse(&built).unwrap();
        number == num && ctype.is_empty() && next
    }

    #[quickcheck]
    fn check_epel_container(number: u32, next: bool) -> bool {
        FedoraRelease::epel(number, ContentType::Container, next).is_err()
    }

    #[quickcheck]
    fn check_epel_flatpak(number: u32, next: bool) -> bool {
        FedoraRelease::epel(number, ContentType::Flatpak, next).is_err()
    }

    #[quickcheck]
    fn check_epel_combo(number: u32, ctype: ContentType) -> bool {
        if number < epel::MIN_RELEASE {
            return true;
        }

        (ctype == ContentType::RPM) != FedoraRelease::epel(number, ctype, true).is_err()
    }
}
