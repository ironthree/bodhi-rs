/// human-readable date format internally used by bodhi
pub const BODHI_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

// https://serde.rs/custom-date-format.html
#[allow(dead_code)]
pub(crate) mod bodhi_date_format {
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
pub(crate) mod option_bodhi_date_format {
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
