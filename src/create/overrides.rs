use std::collections::HashMap;

use serde::Deserialize;

use crate::error::{BodhiError, QueryError};
use crate::{BodhiDate, BodhiService, Build, CSRFQuery, Create, Override, OverrideData};

/// This struct contains the values that are returned when creating a new comment.
#[derive(Debug, Deserialize)]
pub struct NewOverride {
    /// the newly created buildroot override
    #[serde(flatten)]
    pub over_ride: Override,
    /// additional server messages
    pub caveats: Vec<HashMap<String, String>>,
}

/// This struct contains all the values that are necessary for creating a new buildroot override.
/// There are no optional arguments, so everything has to be supplied with the `new()` method.
#[derive(Debug)]
pub struct OverrideBuilder<'a> {
    nvr: &'a str,
    notes: &'a str,
    expiration_date: &'a BodhiDate,
}

impl<'a> OverrideBuilder<'a> {
    /// This method has to be used to create and initialize a new `OverrideBuilder`.
    pub fn new(nvr: &'a str, notes: &'a str, expiration_date: &'a BodhiDate) -> Self {
        OverrideBuilder {
            nvr,
            notes,
            expiration_date,
        }
    }
}

impl<'a> Create<NewOverride> for OverrideBuilder<'a> {
    fn create(&self, bodhi: &BodhiService) -> Result<NewOverride, QueryError> {
        let path = String::from("/overrides/");

        let csrf_token = bodhi.query(CSRFQuery::new())?;

        let new_override = OverrideData {
            nvr: &self.nvr,
            notes: &self.notes,
            expiration_date: &self.expiration_date,
            expired: None,
            edited: None,
            csrf_token: &csrf_token,
        };

        let data = match serde_json::to_string(&new_override) {
            Ok(data) => data,
            Err(error) => return Err(QueryError::SerializationError { error }),
        };

        let response = bodhi.post(&path, data)?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().unwrap_or_else(|_| String::from(""));

            let error: BodhiError = serde_json::from_str(&text)?;
            return Err(QueryError::BodhiError { error });
        };

        let result = response.text()?;

        let new_override: NewOverride = serde_json::from_str(&result)?;

        Ok(new_override)
    }
}

impl Build {
    /// This method creates a new `OverrideBuilder` for this `Build`.
    pub fn buildroot_override<'a>(&'a self, notes: &'a str, expiration_date: &'a BodhiDate) -> OverrideBuilder<'a> {
        OverrideBuilder::new(self.nvr.as_str(), notes, expiration_date)
    }
}
