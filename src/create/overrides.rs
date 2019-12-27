#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::error::{BodhiError, QueryError};
use crate::{BodhiService, CSRFQuery, Create, SinglePageQuery};

/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/overrides.html#service-1-POST>
#[derive(Debug, Serialize)]
struct OverrideData {
    nvr: String,
    notes: String,
    expiration_date: String,
    csrf_token: String,
}

#[derive(Debug, Deserialize)]
pub struct NewOverride {
    comment: u32,
}

#[derive(Debug)]
pub struct OverrideBuilder {
    nvr: String,
    notes: String,
    expiration_date: String,
}

impl OverrideBuilder {
    pub fn new(nvr: String, notes: String, expiration_date: String) -> Self {
        OverrideBuilder {
            nvr,
            notes,
            expiration_date,
        }
    }
}

impl Create<NewOverride> for OverrideBuilder {
    fn create(&self, bodhi: &BodhiService) -> Result<NewOverride, QueryError> {
        let path = String::from("/overrides/");

        let csrf_token = CSRFQuery::new().query(bodhi)?;

        let new_override = OverrideData {
            nvr: self.nvr.clone(),
            notes: self.notes.clone(),
            expiration_date: self.expiration_date.clone(),
            csrf_token,
        };

        let data = match serde_json::to_string(&new_override) {
            Ok(data) => data,
            Err(error) => return Err(QueryError::SerializationError { error }),
        };

        let response = bodhi.post(&path, data)?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().unwrap_or_else(|_| String::from(""));
            println!("{}", &text); // TODO: remove this print once the response contents are clear

            let error: BodhiError = serde_json::from_str(&text)?;
            return Err(QueryError::BodhiError { error });
        };

        let result = response.text()?;

        // TODO
        println!("{}", &result);

        let new_override: NewOverride = serde_json::from_str(&result)?;

        Ok(new_override)
    }
}
