use std::cmp::{Ord, Ordering};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};

/// human-readable, non-standard date format used internally by bodhi servers
pub const BODHI_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// ## newtype wrapper around [`DateTime`] with custom conversion methods
///
/// The bodhi server uses a nonstandard format for datetime values, both in responses and in request
/// parameters. This type is a wrapper around [`DateTime`] with custom implementations for parsing
/// values from strings, formatting values as strings, and (de)serializing values in JSON.
///
/// The format string corresponding to the nonstandard format is defined in
/// [`BODHI_DATETIME_FORMAT`].
#[derive(Clone, Debug, Eq)]
pub struct BodhiDate {
    date: DateTime<Utc>,
}

impl From<DateTime<Utc>> for BodhiDate {
    fn from(date: DateTime<Utc>) -> Self {
        BodhiDate { date }
    }
}

impl TryFrom<&str> for BodhiDate {
    type Error = chrono::ParseError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        // if the string is too short for the full format, pad it with 00:00:00 time.

        let string = if string.len() == 10 {
            format!("{string} 00:00:00")
        } else {
            string.to_owned()
        };

        Ok(BodhiDate {
            date: NaiveDateTime::parse_from_str(&string, BODHI_DATETIME_FORMAT)?.and_utc(),
        })
    }
}

impl FromStr for BodhiDate {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}

impl Display for BodhiDate {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.date.format(BODHI_DATETIME_FORMAT))
    }
}

impl PartialEq for BodhiDate {
    fn eq(&self, other: &Self) -> bool {
        self.date.eq(&other.date)
    }
}

impl PartialOrd for BodhiDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BodhiDate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

// https://serde.rs/custom-date-format.html
#[allow(dead_code)]
pub(crate) mod bodhi_date_format {
    use super::BodhiDate;

    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &BodhiDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = date.date.format(super::BODHI_DATETIME_FORMAT).to_string();
        serializer.serialize_str(&string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BodhiDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        match NaiveDateTime::parse_from_str(&string, super::BODHI_DATETIME_FORMAT) {
            Ok(result) => Ok(BodhiDate { date: result.and_utc() }),
            Err(error) => Err(error).map_err(serde::de::Error::custom),
        }
    }
}

// https://github.com/serde-rs/serde/issues/1444#issuecomment-447546415
#[allow(dead_code)]
pub(crate) mod option_bodhi_date_format_ref {
    use super::BodhiDate;

    use serde::{self, Deserialize, Deserializer, Serializer};

    // this &Option reference is intentional, the API requires it
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(date: &Option<&BodhiDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => super::bodhi_date_format::serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<BodhiDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(with = "super::bodhi_date_format")] BodhiDate);

        let v: Option<Wrapper> = Deserialize::deserialize(deserializer)?;
        Ok(v.map(|Wrapper(a)| a))
    }
}

// https://github.com/serde-rs/serde/issues/1444#issuecomment-447546415
#[allow(dead_code)]
pub(crate) mod option_bodhi_date_format {
    use super::BodhiDate;

    use serde::{self, Deserialize, Deserializer, Serializer};

    // this &Option reference is intentional, the API requires it
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(date: &Option<BodhiDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(ref dt) => super::bodhi_date_format::serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<BodhiDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(with = "super::bodhi_date_format")] BodhiDate);

        let v: Option<Wrapper> = Deserialize::deserialize(deserializer)?;
        Ok(v.map(|Wrapper(a)| a))
    }
}
