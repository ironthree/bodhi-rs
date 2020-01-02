use crate::FedoraRelease;
use std::convert::TryFrom;

#[test]
fn release_try_from_ok() {
    assert_eq!(FedoraRelease::try_from("F31").unwrap(), FedoraRelease::F31);
}

#[test]
#[should_panic]
fn release_try_from_err() {
    FedoraRelease::try_from("X12").unwrap();
}
