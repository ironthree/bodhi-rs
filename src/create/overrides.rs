use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{BodhiError, QueryError};
use crate::{BodhiDate, BodhiService, CSRFQuery, Create};

/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/overrides.html#service-1-POST>
#[derive(Debug, Serialize)]
pub struct OverrideData<'a> {
    nvr: &'a String,
    notes: &'a String,
    #[serde(with = "crate::data::bodhi_date_format")]
    expiration_date: &'a BodhiDate,
    csrf_token: &'a String,
}

/// This struct contains the values that are returned when creating a new comment.
#[derive(Debug, Deserialize)]
pub struct NewOverride {
    // TODO: determine actual fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// This struct contains all the values that are necessary for creating a new buildroot override.
/// There are no optional arguments, so everything has to be supplied with the `new()` method.
#[derive(Debug)]
pub struct OverrideBuilder {
    nvr: String,
    notes: String,
    expiration_date: BodhiDate,
}

impl OverrideBuilder {
    /// This method has to be used to create and initialize a new `OverrideBuilder`.
    pub fn new(nvr: String, notes: String, expiration_date: BodhiDate) -> Self {
        OverrideBuilder {
            nvr,
            notes,
            expiration_date,
        }
    }
}

impl Create<NewOverride> for OverrideBuilder {
    fn create(&self, bodhi: &BodhiService) -> Result<NewOverride, QueryError> {
        // TODO: check if build exists
        let path = String::from("/overrides/");

        let csrf_token = bodhi.query(&CSRFQuery::new())?;

        let new_override = OverrideData {
            nvr: &self.nvr,
            notes: &self.notes,
            expiration_date: &self.expiration_date,
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
