use std::collections::HashMap;

use serde::Deserialize;

use crate::error::{BodhiError, QueryError};
use crate::{BodhiDate, BodhiService, CSRFQuery, Edit, Override, OverrideData};

/// This struct contains the values that are returned when editing a buildroot override.
#[derive(Debug, Deserialize)]
pub struct EditedOverride {
    /// the edited buildroot override
    #[serde(flatten)]
    pub over_ride: Override,
    /// additional server messages
    pub caveats: Vec<HashMap<String, String>>,
}

/// This struct contains all the possible arguments for editing a buildroot override.
#[derive(Debug)]
pub struct OverrideEditor<'a> {
    notes: &'a str,
    expiration_date: &'a BodhiDate,
    expired: Option<bool>,
    /// NVR of the existing buildroot override to edit
    edited: &'a str,
}

impl<'a> OverrideEditor<'a> {
    /// Use this method to create an edit request for an existing buildroot override. It
    /// pre-populates all editable fields with the current values.
    pub fn from_override(over_ride: &'a Override) -> Self {
        OverrideEditor {
            notes: &over_ride.notes,
            expiration_date: &over_ride.expiration_date,
            expired: None,
            edited: &over_ride.nvr,
        }
    }

    /// Change the buildroot override notes.
    pub fn notes(mut self, notes: &'a str) -> Self {
        self.notes = notes;
        self
    }

    /// Change the buildroot override expiration date.
    pub fn expiration_date(mut self, expiration_date: &'a BodhiDate) -> Self {
        self.expiration_date = expiration_date;
        self
    }

    /// Change whether the buildroot override should be expired.
    pub fn expired(mut self, expired: bool) -> Self {
        self.expired = Some(expired);
        self
    }
}

impl<'a> Edit<EditedOverride> for OverrideEditor<'a> {
    fn edit(&self, bodhi: &BodhiService) -> Result<EditedOverride, QueryError> {
        let path = String::from("/overrides/");

        let csrf_token = bodhi.query(&CSRFQuery::new())?;

        let override_edit = OverrideData {
            nvr: self.edited,
            notes: self.notes,
            expiration_date: self.expiration_date,
            expired: self.expired,
            edited: Some(self.edited),
            csrf_token: &csrf_token,
        };

        let data = match serde_json::to_string(&override_edit) {
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
        let edited_override: EditedOverride = serde_json::from_str(&result)?;

        Ok(edited_override)
    }
}

impl Override {
    /// This method creates a new `OverrideEditor` for editing this `Override`.
    pub fn edit(&self) -> OverrideEditor {
        OverrideEditor::from_override(self)
    }
}
