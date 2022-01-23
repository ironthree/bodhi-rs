use serde::Serialize;

use crate::{BodhiDate, UpdateRequest, UpdateSeverity, UpdateSuggestion, UpdateType};

#[derive(Debug, Serialize)]
pub(crate) struct OverrideData<'a> {
    // NVR this buildroot override is filed for
    pub nvr: &'a str,
    // user-visible notes associated with this buildroot override
    pub notes: &'a str,
    // expiration date of this override
    #[serde(with = "crate::data::bodhi_date_format")]
    pub expiration_date: &'a BodhiDate,
    // flag whether this buildroot override is to be expired or not
    pub expired: Option<bool>,
    // NVR of the edited buildroot override if this is an edit request
    pub edited: Option<&'a str>,
    // CSRF token
    pub csrf_token: &'a str,
}

#[derive(Debug, Serialize)]
pub(crate) struct UpdateData<'a> {
    // list of builds to include in the update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds: Option<&'a [&'a str]>,
    // koji side tag to take builds from (if this is specified, builds must be `None` or `[]`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_tag: Option<&'a str>,
    // bugs associated with the update (default: `[]`)
    pub bugs: Option<&'a [&'a str]>,
    // user-visible update title (default: `""`)
    pub display_name: Option<&'a str>,
    // close bugs when update is pushed to stable (default: `true`)
    pub close_bugs: Option<bool>,
    // update type: one of `unspecified`, `bugfix`, `enhancement`, `newpackage`, `security`
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    // update status request (default: `testing`)
    pub request: Option<UpdateRequest>,
    // update severity: one of `unspecified` (default), `low`, `medium`, `high`, `urgent`
    pub severity: Option<UpdateSeverity>,
    // update notes
    pub notes: &'a str,
    // push to stable once `stable_karma` is reached (default: `true`)
    pub autokarma: Option<bool>,
    // stable karma threshold (default: `3`)
    pub stable_karma: Option<i32>,
    // unstable karma threshold (default: `-3`)
    pub unstable_karma: Option<i32>,
    // suggestion after package installation: one of `unspecified` (default), `logout`, `reboot`
    pub suggest: Option<UpdateSuggestion>,
    // alias of the edited update if this is an edit request (default: `""`)
    pub edited: Option<&'a str>,
    // required testcases (comma-separated or space-separated list: default: `""`)
    pub requirements: Option<&'a str>,
    // require bug feedback for karma to be counted (default: `true`)
    pub require_bugs: Option<bool>,
    // require testcase feedback for karma to be counted (default: `true`)
    pub require_testcases: Option<bool>,
    // push update to stable based on time (default: `true`)
    pub autotime: Option<bool>,
    // number of days in testing before the update is pushed to stable automatically (default: `0`)
    pub stable_days: Option<u32>,
    // CSRF token
    pub csrf_token: &'a str,
}
