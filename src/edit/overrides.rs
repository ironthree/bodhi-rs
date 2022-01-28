use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiDate, Override, OverrideData};
use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

/// data of this type is returned after successfully editing a buildroot [`Override`]
#[derive(Debug, Deserialize)]
pub struct EditedOverride {
    /// edited buildroot override
    #[serde(flatten)]
    pub over_ride: Override,
    /// additional server messages
    pub caveats: Vec<HashMap<String, String>>,

    // private field that makes it impossible to construct values of this type outside this crate
    #[serde(skip)]
    #[allow(dead_code)]
    pub(crate) private: (),
}


/// data type wrapping all mandatory and optional parameters for editing a buildroot override
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/overrides.html#service-1-POST>
#[derive(Debug)]
pub struct OverrideEditor<'a> {
    notes: &'a str,
    expiration_date: &'a BodhiDate,
    expired: Option<bool>,
    // NVR of the existing buildroot override to edit
    edited: &'a str,
}

impl<'a> OverrideEditor<'a> {
    /// constructor for [`OverrideEditor`] from an existing [`Override`] value
    pub fn from_override(over_ride: &'a Override) -> Self {
        OverrideEditor {
            notes: &over_ride.notes,
            expiration_date: &over_ride.expiration_date,
            expired: None,
            edited: &over_ride.nvr,
        }
    }

    /// method for changing the override notes
    #[must_use]
    pub fn notes(mut self, notes: &'a str) -> Self {
        self.notes = notes;
        self
    }

    /// method for changing the expiration date of the override
    #[must_use]
    pub fn expiration_date(mut self, expiration_date: &'a BodhiDate) -> Self {
        self.expiration_date = expiration_date;
        self
    }

    /// method for setting whether the override should be expired
    #[must_use]
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
            csrf_token: csrf_token.as_ref().unwrap_or_else(|| unreachable!()),
        };

        Ok(Some(
            serde_json::to_string(&override_edit).map_err(|error| QueryError::SerializationError { error })?,
        ))
    }

    fn parse(&self, string: &str) -> Result<EditedOverride, QueryError> {
        let edited_override: EditedOverride = serde_json::from_str(string)?;
        Ok(edited_override)
    }

    fn extract(&self, page: EditedOverride) -> EditedOverride {
        page
    }
}


impl Override {
    /// constructor for [`OverrideEditor`] which takes the NVR from an existing [`Override`]
    pub fn edit(&self) -> OverrideEditor {
        OverrideEditor::from_override(self)
    }
}
