use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiDate, Build, Override, OverrideData};
use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

// This struct contains the values that are returned when creating a new comment.
#[derive(Debug, Deserialize)]
pub struct NewOverride {
    // the newly created buildroot override
    #[serde(flatten)]
    pub over_ride: Override,
    // additional server messages
    pub caveats: Vec<HashMap<String, String>>,
}

// This struct contains all the values that are necessary for creating a new buildroot override.
// There are no optional arguments, so everything has to be supplied with the `new()` method.
#[derive(Debug)]
pub struct OverrideCreator<'a> {
    nvr: &'a str,
    notes: &'a str,
    expiration_date: &'a BodhiDate,
}

impl<'a> OverrideCreator<'a> {
    // This method has to be used to create and initialize a new `OverrideBuilder`.
    pub fn new(nvr: &'a str, notes: &'a str, expiration_date: &'a BodhiDate) -> Self {
        OverrideCreator {
            nvr,
            notes,
            expiration_date,
        }
    }
}

impl<'a> SingleRequest<NewOverride, NewOverride> for OverrideCreator<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::POST
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/overrides/"))
    }

    fn body(&self, csrf_token: Option<String>) -> Result<Option<String>, QueryError> {
        let new_override = OverrideData {
            nvr: self.nvr,
            notes: self.notes,
            expiration_date: self.expiration_date,
            expired: None,
            edited: None,
            csrf_token: csrf_token.as_ref().unwrap_or_else(|| unreachable!()),
        };

        match serde_json::to_string(&new_override) {
            Ok(result) => Ok(Some(result)),
            Err(error) => Err(QueryError::SerializationError { error }),
        }
    }

    fn parse(&self, string: &str) -> Result<NewOverride, QueryError> {
        let new_override: NewOverride = serde_json::from_str(string)?;
        Ok(new_override)
    }

    fn extract(&self, page: NewOverride) -> NewOverride {
        page
    }
}

impl Build {
    // This method creates a new `OverrideBuilder` for this `Build`.
    pub fn buildroot_override<'a>(&'a self, notes: &'a str, expiration_date: &'a BodhiDate) -> OverrideCreator<'a> {
        OverrideCreator::new(self.nvr.as_str(), notes, expiration_date)
    }
}
