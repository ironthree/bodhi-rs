use serde::{Deserialize, Serialize};

use crate::create::Create;
use crate::data::Update;
use crate::error::{BodhiError, QueryError};
use crate::query::{CSRFQuery, SinglePageQuery};
use crate::service::BodhiService;

// https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-2-POST
#[derive(Debug, Serialize)]
struct UpdateData {
    /// list of builds to include in the update
    builds: Option<Vec<String>>,
    /// koji side tag to take builds from (if this is specified, builds must be `None` or `[]`)
    from_tag: Option<String>,
    /// bugs associated with the update (default: `[]`)
    bugs: Option<Vec<u32>>,
    /// user-visible update title (default: `""`)
    display_name: Option<String>,
    /// close bugs when update is pushed to stable (default: `true`)
    close_bugs: Option<bool>,
    /// update type: one of `unspecified`, `bugfix`, `enhancement`, `newpackage`, `security`
    r#type: String,
    /// update status request (default: `testing`)
    request: Option<String>,
    /// update severity: one of `unspecified` (default), `low`, `medium`, `high`, `urgent`
    severity: Option<String>,
    /// update notes
    notes: String,
    /// push to stable once `stable_karma` is reached (default: `true`)
    autokarma: Option<bool>,
    /// stable karma threshold (default: `3`)
    stable_karma: Option<i32>,
    /// unstable karma threshold (default: `-3`)
    unstable_karma: Option<i32>,
    /// suggestion after package installation: one of `unspecified` (default), `logout`, `reboot`
    suggest: Option<String>,
    /// alias of the edited update if this is an edit request (default: `""`)
    edited: Option<String>,
    /// required testcases (comma-separated or space-separated list: default: `""`)
    requirements: Option<String>,
    /// require bug feedback for karma to be counted (default: `true`)
    require_bugs: Option<bool>,
    /// require testcase feedback for karma to be counted (default: `true`)
    require_testcases: Option<bool>,
    /// push update to stable based on time (default: `true`)
    autotime: Option<bool>,
    /// number of days in testing before the update is pushed to stable automatically (default: `0`)
    stable_days: Option<u32>,
    /// CSRF token
    csrf_token: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUpdate {}

#[derive(Debug)]
pub struct UpdateBuilder {}

impl UpdateBuilder {}

impl Create<NewUpdate> for UpdateBuilder {
    fn create(&self, bodhi: &BodhiService) -> Result<NewUpdate, QueryError> {
        todo!()
    }
}
