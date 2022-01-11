use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiDate, Override, OverrideData};
use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

// This struct contains the values that are returned when editing a buildroot override.
#[derive(Debug, Deserialize)]
pub struct EditedOverride {
    // the edited buildroot override
    #[serde(flatten)]
    pub over_ride: Override,
    // additional server messages
    pub caveats: Vec<HashMap<String, String>>,
}

// This struct contains all the possible arguments for editing a buildroot override.
#[derive(Debug)]
pub struct OverrideEditor<'a> {
    notes: &'a str,
    expiration_date: &'a BodhiDate,
    expired: Option<bool>,
    // NVR of the existing buildroot override to edit
    edited: &'a str,
}

impl<'a> OverrideEditor<'a> {
    // Use this method to create an edit request for an existing buildroot override. It
    // pre-populates all editable fields with the current values.
    pub fn from_override(over_ride: &'a Override) -> Self {
        OverrideEditor {
            notes: &over_ride.notes,
            expiration_date: &over_ride.expiration_date,
            expired: None,
            edited: &over_ride.nvr,
        }
    }

    // Change the buildroot override notes.
    pub fn notes(mut self, notes: &'a str) -> Self {
        self.notes = notes;
        self
    }

    // Change the buildroot override expiration date.
    pub fn expiration_date(mut self, expiration_date: &'a BodhiDate) -> Self {
        self.expiration_date = expiration_date;
        self
    }

    // Change whether the buildroot override should be expired.
    pub fn expired(mut self, expired: bool) -> Self {
        self.expired = Some(expired);
        self
    }
}

impl<'a> SingleRequest<EditedOverride, EditedOverride> for OverrideEditor<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::POST
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/overrides/"))
    }

    fn body(&self, csrf_token: Option<String>) -> Result<Option<String>, QueryError> {
        let override_edit = OverrideData {
            nvr: self.edited,
            notes: self.notes,
            expiration_date: self.expiration_date,
            expired: self.expired,
            edited: Some(self.edited),
            csrf_token: csrf_token.as_ref().unwrap(),
        };

        match serde_json::to_string(&override_edit) {
            Ok(result) => Ok(Some(result)),
            Err(error) => Err(QueryError::SerializationError { error }),
        }
    }

    fn parse(&self, string: &str) -> Result<EditedOverride, QueryError> {
        let edited_override: EditedOverride = serde_json::from_str(&string)?;
        Ok(edited_override)
    }

    fn extract(&self, page: EditedOverride) -> EditedOverride {
        page
    }
}

impl Override {
    // This method creates a new `OverrideEditor` for editing this `Override`.
    pub fn edit(&self) -> OverrideEditor {
        OverrideEditor::from_override(self)
    }
}
