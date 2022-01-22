use thiserror::Error;

/// error type that represents an attempt to parse invalid data
#[derive(Debug, Error)]
#[error("Invalid value for {}: {}", .name, .value)]
pub struct InvalidValueError {
    /// target type the string failed to parse as
    pub name: &'static str,
    /// value that failed to parse
    pub value: String,
}

impl InvalidValueError {
    pub(crate) fn new(name: &'static str, value: String) -> Self {
        InvalidValueError { name, value }
    }
}
