use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::{Update, UpdateData, UpdateRequest, UpdateSeverity, UpdateSuggestion, UpdateType};
use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

/// data of this type is returned after successfully editing an [`Update`]
#[derive(Debug, Deserialize)]
pub struct EditedUpdate {
    /// edited update
    #[serde(flatten)]
    pub update: Update,
    /// additional server messages
    pub caveats: Vec<HashMap<String, String>>,

    // private field that makes it impossible to construct values of this type outside this crate
    #[serde(skip)]
    #[allow(dead_code)]
    pub(crate) private: (),
}


/// data type wrapping all mandatory and optional parameters for editing an update
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-2-POST>
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
    /// constructor for [`UpdateEditor`] from an existing [`Update`]] value
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

    /// method for adding a build to the update
    #[must_use]
    pub fn add_build(mut self, build: &'a str) -> Self {
        self.builds.push(build);
        self
    }

    /// method for removing a build from the update
    #[must_use]
    pub fn remove_build(mut self, build: &'a str) -> Self {
        self.builds.retain(|b| *b != build);
        self
    }

    /// method for changing the update notes
    #[must_use]
    pub fn notes(mut self, notes: &'a str) -> Self {
        self.notes = notes;
        self
    }

    /// method for adding a related bug to the update
    #[must_use]
    pub fn add_bug(mut self, bug: u32) -> Self {
        self.bugs.push(bug);
        self
    }

    /// method for removing a related bug from the update
    #[must_use]
    pub fn remove_bug(mut self, bug: u32) -> Self {
        self.bugs.retain(|b| *b != bug);
        self
    }

    /// method for changing the "pretty" update title
    #[must_use]
    pub fn display_name(mut self, display_name: &'a str) -> Self {
        self.display_name = Some(display_name);
        self
    }

    /// method for changing the `close_bugs` flag
    #[must_use]
    pub fn close_bugs(mut self, close_bugs: bool) -> Self {
        self.close_bugs = Some(close_bugs);
        self
    }

    /// method for changing the update type
    ///
    /// Note that updates of type [`UpdateType::Security`] also need a severity value that is not
    /// [`UpdateSeverity::Unspecified`].
    #[must_use]
    pub fn update_type(mut self, update_type: UpdateType) -> Self {
        self.update_type = Some(update_type);
        self
    }

    /// method for changing the update severity
    #[must_use]
    pub fn severity(mut self, severity: UpdateSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// method for changing the `autokarma` flag
    #[must_use]
    pub fn autokarma(mut self, autokarma: bool) -> Self {
        self.autokarma = Some(autokarma);
        self
    }

    /// method for changing the stable karma threshold
    #[must_use]
    pub fn stable_karma(mut self, stable_karma: i32) -> Self {
        self.stable_karma = Some(stable_karma);
        self
    }

    /// method for changing the unstable karma threshold
    #[must_use]
    pub fn unstable_karma(mut self, unstable_karma: i32) -> Self {
        self.unstable_karma = Some(unstable_karma);
        self
    }

    /// method for changing the update suggestion
    #[must_use]
    pub fn suggest(mut self, suggestion: UpdateSuggestion) -> Self {
        self.suggest = Some(suggestion);
        self
    }

    /// method for changing the required gating tests
    #[must_use]
    pub fn requirements(mut self, requirements: &'a str) -> Self {
        self.requirements = Some(requirements);
        self
    }

    /// method for changing the `require_bugs` flag
    #[must_use]
    pub fn require_bugs(mut self, require_bugs: bool) -> Self {
        self.require_bugs = Some(require_bugs);
        self
    }

    /// method for changing the `require_testcases` flag
    #[must_use]
    pub fn require_testcases(mut self, require_testcases: bool) -> Self {
        self.require_testcases = Some(require_testcases);
        self
    }

    /// method for changing the `autotime` flag
    #[must_use]
    pub fn autotime(mut self, autotime: bool) -> Self {
        self.autotime = Some(autotime);
        self
    }

    /// method for changing the stable time threshold
    #[must_use]
    pub fn stable_days(mut self, stable_days: u32) -> Self {
        self.stable_days = Some(stable_days);
        self
    }
}

impl<'a> SingleRequest<EditedUpdate, EditedUpdate> for UpdateEditor<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::POST
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/updates/"))
    }

    fn body(&self, csrf_token: Option<String>) -> Result<Option<String>, QueryError> {
        // do some data sanity verification
        if matches!(self.stable_karma, Some(karma) if karma < 1) {
            return Err(QueryError::InvalidDataError {
                error: String::from("Stable karma must be positive."),
            });
        }

        if matches!(self.unstable_karma, Some(karma) if karma > -1) {
            return Err(QueryError::InvalidDataError {
                error: String::from("Unstable karma must be negative."),
            });
        }

        if matches!(
            (self.update_type, self.severity),
            (Some(UpdateType::Security), Some(UpdateSeverity::Unspecified) | None)
        ) {
            return Err(QueryError::InvalidDataError {
                error: String::from("For security updates, severity has to be specified."),
            });
        }

        let bugs: Vec<String> = self.bugs.iter().map(|b| format!("{b}")).collect();
        let bug_refs: Vec<&str> = bugs.iter().map(|s| s.as_str()).collect();

        let update_edit = UpdateData {
            builds: Some(&self.builds),
            from_tag: None,
            bugs: Some(bug_refs.as_slice()),
            display_name: self.display_name,
            close_bugs: self.close_bugs,
            update_type: match self.update_type {
                Some(t) => t,
                None => UpdateType::Unspecified,
            },
            request: self.request,
            severity: self.severity,
            notes: self.notes,
            autokarma: self.autokarma,
            stable_karma: self.stable_karma,
            unstable_karma: self.unstable_karma,
            suggest: self.suggest,
            edited: match &self.edited {
                Some(string) => Some(string),
                None => None,
            },
            requirements: self.requirements,
            require_bugs: self.require_bugs,
            require_testcases: self.require_testcases,
            autotime: self.autotime,
            stable_days: self.stable_days,
            csrf_token: csrf_token.as_ref().unwrap_or_else(|| unreachable!()),
        };

        Ok(Some(
            serde_json::to_string(&update_edit).map_err(|error| QueryError::SerializationError { error })?,
        ))
    }

    fn parse(&self, string: &str) -> Result<EditedUpdate, QueryError> {
        let edited_update: EditedUpdate = serde_json::from_str(string)?;
        Ok(edited_update)
    }

    fn extract(&self, page: EditedUpdate) -> EditedUpdate {
        page
    }
}


#[derive(Debug, Deserialize)]
pub struct RequestedUpdate {
    update: Update,
}


/// data type wrapping all mandatory arguments for creating a request to change an update status
#[derive(Debug)]
pub struct UpdateStatusRequester<'a> {
    alias: &'a str,
    request: UpdateRequest,
}

impl<'a> UpdateStatusRequester<'a> {
    /// constructor for [`UpdateStatusRequester`] from an existing [`Update`] value
    pub fn from_update(update: &'a Update, request: UpdateRequest) -> Self {
        UpdateStatusRequester {
            alias: &update.alias,
            request,
        }
    }
}

impl<'a> SingleRequest<RequestedUpdate, Update> for UpdateStatusRequester<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::POST
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/updates/{}/request", &self.alias))
    }

    fn body(&self, csrf_token: Option<String>) -> Result<Option<String>, QueryError> {
        #[derive(Serialize)]
        struct RequestEdit<'a> {
            request: UpdateRequest,
            csrf_token: &'a str,
        }

        let request_edit = RequestEdit {
            request: self.request,
            csrf_token: csrf_token.as_ref().unwrap_or_else(|| unreachable!()),
        };

        Ok(Some(
            serde_json::to_string(&request_edit).map_err(|error| QueryError::SerializationError { error })?,
        ))
    }

    fn parse(&self, string: &str) -> Result<RequestedUpdate, QueryError> {
        let requested_update: RequestedUpdate = serde_json::from_str(string)?;
        Ok(requested_update)
    }

    fn extract(&self, page: RequestedUpdate) -> Update {
        page.update
    }
}


#[derive(Debug, Deserialize)]
pub struct WaivedUpdate {
    update: Update,
}


/// data type wrapping all mandatory arguments for creating a request to waive test results
#[derive(Debug)]
pub struct UpdateTestResultWaiver<'a> {
    alias: &'a str,
    comment: &'a str,
    tests: Option<&'a [&'a str]>,
}

impl<'a> UpdateTestResultWaiver<'a> {
    /// constructor for [`UpdateTestResultWaiver`] from an existing [`Update`] value
    pub fn from_update(update: &'a Update, comment: &'a str) -> Self {
        UpdateTestResultWaiver {
            alias: &update.alias,
            comment,
            tests: None,
        }
    }

    /// method for setting the tests for which results should be waived
    ///
    /// If no tests are explicitly specified by using this method, all test results are waived.
    #[must_use]
    pub fn tests(mut self, tests: &'a [&'a str]) -> Self {
        self.tests = Some(tests);
        self
    }
}

impl<'a> SingleRequest<WaivedUpdate, Update> for UpdateTestResultWaiver<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::POST
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/updates/{}/waive-test-results", &self.alias))
    }

    fn body(&self, csrf_token: Option<String>) -> Result<Option<String>, QueryError> {
        #[derive(Serialize)]
        struct RequestWaiver<'a> {
            comment: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            tests: Option<&'a [&'a str]>,
            csrf_token: &'a str,
        }

        let request_waiver = RequestWaiver {
            comment: self.comment,
            tests: self.tests,
            csrf_token: csrf_token.as_ref().unwrap_or_else(|| unreachable!()),
        };

        Ok(Some(
            serde_json::to_string(&request_waiver).map_err(|error| QueryError::SerializationError { error })?,
        ))
    }

    fn parse(&self, string: &str) -> Result<WaivedUpdate, QueryError> {
        let waived_update: WaivedUpdate = serde_json::from_str(string)?;
        Ok(waived_update)
    }

    fn extract(&self, page: WaivedUpdate) -> Update {
        page.update
    }
}


impl Update {
    /// constructor for [`UpdateEditor`] which takes parameters from an existing [`Update`]
    pub fn edit(&self) -> UpdateEditor {
        UpdateEditor::from_update(self)
    }

    /// constructor for [`UpdateStatusRequester`] which takes parameters from an existing [`Update`]
    pub fn request(&self, request: UpdateRequest) -> UpdateStatusRequester {
        UpdateStatusRequester::from_update(self, request)
    }

    /// constructor for [`UpdateTestResultWaiver`] which takes parameters from an existing
    /// [`Update`]
    pub fn waive<'a>(&'a self, comment: &'a str) -> UpdateTestResultWaiver<'a> {
        UpdateTestResultWaiver::from_update(self, comment)
    }
}
