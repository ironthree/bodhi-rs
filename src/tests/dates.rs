use crate::BodhiDate;

#[test]
fn long_date() {
    BodhiDate::try_from("2020-01-01 00:00:00").unwrap();
}

#[test]
fn short_date() {
    BodhiDate::try_from("2020-01-01").unwrap();
}

#[test]
fn idem() {
    let string = String::from("2020-01-01 00:00:00");
    assert_eq!(string.parse::<BodhiDate>().unwrap().to_string(), string);
}
