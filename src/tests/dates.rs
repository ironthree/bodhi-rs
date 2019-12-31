use crate::BodhiDate;
use std::convert::TryFrom;

#[test]
fn long_date() {
    BodhiDate::try_from("2020-01-01 00:00:00").unwrap();
}

#[test]
fn short_date() {
    BodhiDate::try_from("2020-01-01").unwrap();
}
