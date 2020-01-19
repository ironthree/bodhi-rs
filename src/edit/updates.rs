use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{BodhiError, QueryError};
use crate::{
    BodhiService,
    CSRFQuery,
    Edit,
    Update,
    UpdateData,
    UpdateRequest,
    UpdateSeverity,
    UpdateSuggestion,
    UpdateType,
};

/// This struct contains the values that are returned when editing an update.
#[derive(Debug, Deserialize)]
pub struct EditedUpdate {
    /// the edited update
    #[serde(flatten)]
    pub update: Update,
    /// additional server messages
    pub caveats: Vec<HashMap<String, String>>,
}

/// This struct contains all the possible arguments for editing an update. Methods to supply
/// optional arguments are also available.
#[derive(Debug)]
pub struct UpdateEditor<'a> {
    // mandatory fields
    builds: Vec<&'a str>,
    notes: &'a str,

    // optional fields
    bugs: Vec<u32>,
    display_name: Option<&'a str>,
    close_bugs: Option<bool>,
    update_type: Option<UpdateType>,
    request: Option<UpdateRequest>,
    severity: Option<UpdateSeverity>,
    autokarma: Option<bool>,
    stable_karma: Option<i32>,
    unstable_karma: Option<i32>,
    suggest: Option<UpdateSuggestion>,
    edited: Option<&'a str>,
    requirements: Option<&'a str>,
    require_bugs: Option<bool>,
    require_testcases: Option<bool>,
    autotime: Option<bool>,
    stable_days: Option<u32>,
}

impl<'a> UpdateEditor<'a> {
    /// Use this method to create an edit request for an existing update. It pre-populates all
    /// fields with the current values.
    pub fn from_update(update: &'a Update) -> Self {
        UpdateEditor {
            builds: update.builds.iter().map(|b| b.nvr.as_str()).collect(),
            notes: &update.notes,

            bugs: update.bugs.iter().map(|bug| bug.bug_id).collect(),
            display_name: Some(&update.display_name),
            close_bugs: Some(update.close_bugs),
            update_type: Some(update.update_type),
            request: update.request,
            severity: Some(update.severity),
            autokarma: Some(update.autokarma),
            stable_karma: update.stable_karma,
            unstable_karma: update.unstable_karma,
            suggest: Some(update.suggest),
            edited: Some(&update.alias),
            requirements: match &update.requirements {
                Some(string) => Some(string),
                None => None,
            },
            require_bugs: Some(update.require_bugs),
            require_testcases: Some(update.require_testcases),
            autotime: Some(update.autotime),
            stable_days: update.stable_days,
        }
    }

    /// Add a build to the update.
    pub fn add_build(mut self, build: &'a str) -> Self {
        self.builds.push(build);
        self
    }

    /// Remove a build to the update.
    pub fn remove_build(mut self, build: &'a str) -> Self {
        self.builds.retain(|b| *b != build);
        self
    }

    /// Change the update notes.
    pub fn notes(mut self, notes: &'a str) -> Self {
        self.notes = notes;
        self
    }

    /// Add a related bug to the update.
    pub fn add_bug(mut self, bug: u32) -> Self {
        self.bugs.push(bug);
        self
    }

    /// Remove a related bug from the update.
    pub fn remove_bug(mut self, bug: u32) -> Self {
        self.bugs.retain(|b| *b != bug);
        self
    }

    /// Change the custom, user-visible title of the update.
    pub fn display_name(mut self, display_name: &'a str) -> Self {
        self.display_name = Some(display_name);
        self
    }

    /// Set the flag whether bugs will be closed when the update is pushed to stable.
    pub fn close_bugs(mut self, close_bugs: bool) -> Self {
        self.close_bugs = Some(close_bugs);
        self
    }

    /// Flag to specify the type of update (new package, bug fix, enhancement, security update, or
    /// unspecified). For security updates, the severity also has to be specified.
    pub fn update_type(mut self, update_type: UpdateType) -> Self {
        self.update_type = Some(update_type);
        self
    }

    /// Flag to specify the update severity (primarily used for security updates, where this flag is
    /// mandatory).
    pub fn severity(mut self, severity: UpdateSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Set the flag whether the update can automatically be pushed to stable once it reaches the
    /// specified stable karma.
    pub fn autokarma(mut self, autokarma: bool) -> Self {
        self.autokarma = Some(autokarma);
        self
    }

    /// Manually set the stable karma feedback threshold.
    pub fn stable_karma(mut self, stable_karma: i32) -> Self {
        self.stable_karma = Some(stable_karma);
        self
    }

    /// Manually set the unstable karma feedback threshold.
    pub fn unstable_karma(mut self, unstable_karma: i32) -> Self {
        self.unstable_karma = Some(unstable_karma);
        self
    }

    /// Flag to specify whether users should log out or reboot to successfully apply an update.
    pub fn suggest(mut self, suggestion: UpdateSuggestion) -> Self {
        self.suggest = Some(suggestion);
        self
    }

    /// Set custom taskotron requirements.
    pub fn requirements(mut self, requirements: &'a str) -> Self {
        self.requirements = Some(requirements);
        self
    }

    /// Flag to indicate whether bug feedback is required for karma to be counted.
    pub fn require_bugs(mut self, require_bugs: bool) -> Self {
        self.require_bugs = Some(require_bugs);
        self
    }

    /// Flag to indicate whether test case feedback is required for karma to be counted.
    pub fn require_testcases(mut self, require_testcases: bool) -> Self {
        self.require_testcases = Some(require_testcases);
        self
    }

    /// Set the flag whether the update can automatically be pushed to stable once it reaches the
    /// specified days in testing.
    pub fn autotime(mut self, autotime: bool) -> Self {
        self.autotime = Some(autotime);
        self
    }

    /// Manually specify the minimum duration the update has to stay in testing.
    ///
    /// The default is 7 days for stable updates, 14 days for stable updates containing critpath
    /// packages, and 3 days for fedora pre-releases.
    pub fn stable_days(mut self, stable_days: u32) -> Self {
        self.stable_days = Some(stable_days);
        self
    }
}

impl<'a> Edit<EditedUpdate> for UpdateEditor<'a> {
    fn edit(&self, bodhi: &BodhiService) -> Result<EditedUpdate, QueryError> {
        let path = String::from("/updates/");

        // do some data sanity verification
        if let Some(karma) = self.stable_karma {
            if karma < 1 {
                return Err(QueryError::InvalidDataError {
                    error: String::from("Stable karma must be positive."),
                });
            }
        }
        if let Some(karma) = self.unstable_karma {
            if karma > -1 {
                return Err(QueryError::InvalidDataError {
                    error: String::from("Unstable karma must be negative."),
                });
            }
        }

        if let Some(update_type) = self.update_type {
            if update_type == UpdateType::Security {
                match self.severity {
                    Some(value) if value == UpdateSeverity::Unspecified => {
                        return Err(QueryError::InvalidDataError {
                            error: String::from("For security updates, severity has to be specified."),
                        });
                    },
                    None => {
                        return Err(QueryError::InvalidDataError {
                            error: String::from("For security updates, severity has to be specified."),
                        });
                    },
                    _ => {},
                }
            }
        }

        let csrf_token = bodhi.query(CSRFQuery::new())?;

        let bugs: Vec<String> = self.bugs.iter().map(|b| format!("{}", b)).collect();

        let update_edit = UpdateData {
            builds: Some(&self.builds),
            from_tag: None,
            bugs: Some(&bugs),
            display_name: self.display_name,
            close_bugs: self.close_bugs,
            update_type: match self.update_type {
                Some(t) => t,
                None => UpdateType::Unspecified,
            },
            request: self.request,
            severity: self.severity,
            notes: &self.notes,
            autokarma: self.autokarma,
            stable_karma: self.stable_karma,
            unstable_karma: self.unstable_karma,
            suggest: self.suggest,
            edited: match &self.edited {
                Some(string) => Some(&string),
                None => None,
            },
            requirements: self.requirements,
            require_bugs: self.require_bugs,
            require_testcases: self.require_testcases,
            autotime: self.autotime,
            stable_days: self.stable_days,
            csrf_token: &csrf_token,
        };

        let data = match serde_json::to_string(&update_edit) {
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
        let edited_update: EditedUpdate = serde_json::from_str(&result)?;

        Ok(edited_update)
    }
}


#[derive(Debug, Deserialize)]
struct RequestedUpdate {
    update: Update,
}

/// This struct contains all the arguments for changing the update status request.
#[derive(Debug)]
pub struct UpdateStatusRequester<'a> {
    alias: &'a str,
    request: UpdateRequest,
}

impl<'a> UpdateStatusRequester<'a> {
    /// Use this method when creating an update state request.
    pub fn from_update(update: &'a Update, request: UpdateRequest) -> Self {
        UpdateStatusRequester {
            alias: &update.alias,
            request,
        }
    }
}

impl<'a> Edit<Update> for UpdateStatusRequester<'a> {
    fn edit(&self, bodhi: &BodhiService) -> Result<Update, QueryError> {
        let path = format!("/updates/{}/request", &self.alias);

        let csrf_token = bodhi.query(CSRFQuery::new())?;

        #[derive(Serialize)]
        struct RequestEdit<'a> {
            request: UpdateRequest,
            csrf_token: &'a str,
        }

        let request_edit = RequestEdit {
            request: self.request,
            csrf_token: &csrf_token,
        };

        let data = match serde_json::to_string(&request_edit) {
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
        let requested_update: RequestedUpdate = serde_json::from_str(&result)?;

        Ok(requested_update.update)
    }
}


#[derive(Debug, Deserialize)]
struct WaivedUpdate {
    update: Update,
}

/// This struct contains all the arguments for waiving test results for an update.
#[derive(Debug)]
pub struct UpdateTestResultWaiver<'a> {
    alias: &'a str,
    comment: &'a str,
}

impl<'a> UpdateTestResultWaiver<'a> {
    /// Use this method when creating the waive request.
    pub fn from_update(update: &'a Update, comment: &'a str) -> Self {
        UpdateTestResultWaiver {
            alias: &update.alias,
            comment,
        }
    }
}

impl<'a> Edit<Update> for UpdateTestResultWaiver<'a> {
    fn edit(&self, bodhi: &BodhiService) -> Result<Update, QueryError> {
        let path = format!("/updates/{}/waive-test-results", &self.alias);

        let csrf_token = bodhi.query(CSRFQuery::new())?;

        #[derive(Serialize)]
        struct RequestWaiver<'a> {
            comment: &'a str,
            // tests: ?
            csrf_token: &'a str,
        }

        let request_waiver = RequestWaiver {
            comment: self.comment,
            // tests: ?
            csrf_token: &csrf_token,
        };

        let data = match serde_json::to_string(&request_waiver) {
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
        let waived_update: WaivedUpdate = serde_json::from_str(&result)?;

        Ok(waived_update.update)
    }
}

impl Update {
    /// This method creates a new `UpdateEditor` for editing this `Update`.
    pub fn edit(&self) -> UpdateEditor {
        UpdateEditor::from_update(self)
    }

    /// This method creates a new `UpdateStatusRequester` for editing this `Update`.
    pub fn request(&self, request: UpdateRequest) -> UpdateStatusRequester {
        UpdateStatusRequester::from_update(self, request)
    }

    /// This method creates a new `UpdateTestResultWaiver` for editing this `Update`.
    pub fn waive<'a>(&'a self, comment: &'a str) -> UpdateTestResultWaiver<'a> {
        UpdateTestResultWaiver::from_update(self, comment)
    }
}
