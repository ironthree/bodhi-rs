use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{Update, UpdateData, UpdateRequest, UpdateSeverity, UpdateSuggestion, UpdateType};
use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

// imports for intra-doc links
#[cfg(doc)]
use crate::data::UpdateStatus;

/// data of this type is returned after successfully creating a new [`Update`]
#[derive(Debug, Deserialize)]
pub struct NewUpdate {
    /// new update that was just created
    #[serde(flatten)]
    pub update: Update,
    /// additional server messages
    pub caveats: Vec<HashMap<String, String>>,

    // private field that makes it impossible to construct values of this type outside this crate
    #[serde(skip)]
    #[allow(dead_code)]
    pub(crate) private: (),
}

#[derive(Debug)]
enum UpdateSource<'a> {
    Builds { builds: &'a [&'a str] },
    Tag { tag: &'a str },
}

/// data type wrapping all mandatory and optional parameters for creating a new update
///
/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/updates.html#service-2-POST>
#[derive(Debug)]
pub struct UpdateCreator<'a> {
    // mandatory fields
    source: UpdateSource<'a>,
    notes: &'a str,

    // optional fields
    bugs: Option<&'a [u32]>,
    display_name: Option<&'a str>,
    close_bugs: Option<bool>,
    update_type: Option<UpdateType>,
    request: Option<UpdateRequest>,
    severity: Option<UpdateSeverity>,
    autokarma: Option<bool>,
    stable_karma: Option<i32>,
    unstable_karma: Option<i32>,
    suggest: Option<UpdateSuggestion>,
    requirements: Option<&'a str>,
    require_bugs: Option<bool>,
    require_testcases: Option<bool>,
    autotime: Option<bool>,
    stable_days: Option<u32>,
}

impl<'a> UpdateCreator<'a> {
    /// constructor for [`UpdateCreator`] with a list of build NVRs and update notes as mandatory
    /// parameters, and default values for all optional parameters
    pub fn from_builds(builds: &'a [&str], notes: &'a str) -> Self {
        UpdateCreator {
            source: UpdateSource::Builds { builds },
            notes,

            bugs: None,
            display_name: None,
            close_bugs: None,
            update_type: None,
            request: None,
            severity: None,
            autokarma: None,
            stable_karma: None,
            unstable_karma: None,
            suggest: None,
            requirements: None,
            require_bugs: None,
            require_testcases: None,
            autotime: None,
            stable_days: None,
        }
    }

    /// constructor for [`UpdateCreator`] with the name of a koji side tag as mandatory argument,
    /// and default values for all optional parameters
    pub fn from_tag(tag: &'a str, notes: &'a str) -> Self {
        UpdateCreator {
            source: UpdateSource::Tag { tag },
            notes,

            bugs: None,
            display_name: None,
            close_bugs: None,
            update_type: None,
            request: None,
            severity: None,
            autokarma: None,
            stable_karma: None,
            unstable_karma: None,
            suggest: None,
            requirements: None,
            require_bugs: None,
            require_testcases: None,
            autotime: None,
            stable_days: None,
        }
    }

    /// method for setting the optional list of associated bugs
    #[must_use]
    pub fn bugs(mut self, bugs: &'a [u32]) -> Self {
        self.bugs = Some(bugs);
        self
    }

    /// method for setting the optional preference whether associated bugs should be closed when
    /// an update is pushed to stable or not
    #[must_use]
    pub fn close_bugs(mut self, close_bugs: bool) -> Self {
        self.close_bugs = Some(close_bugs);
        self
    }

    /// method for setting an optional "pretty" display name that will be used in the bodhi web UI
    /// instead of a name that is automatically generated from the list of builds in the update
    #[must_use]
    pub fn display_name(mut self, display_name: &'a str) -> Self {
        self.display_name = Some(display_name);
        self
    }

    /// method for optionally setting the update type to a specific value
    ///
    /// If no value is specified for the update type, the server will create it with a default
    /// value of [`UpdateType::Unspecified`].
    #[must_use]
    pub fn update_type(mut self, update_type: UpdateType) -> Self {
        self.update_type = Some(update_type);
        self
    }

    /// method for optionally setting the update severity to a specific value
    ///
    /// If no value is specified for the update severity, the server will create it with a default
    /// value of [`UpdateSeverity::Unspecified`].
    #[must_use]
    pub fn severity(mut self, severity: UpdateSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// method for setting the optional preference whether an update should be pushed to stable
    /// after receiving total karma that is equal to or greater than the `stable_karma` value
    #[must_use]
    pub fn autokarma(mut self, autokarma: bool) -> Self {
        self.autokarma = Some(autokarma);
        self
    }

    /// method for optionally overriding the default stable karma threshold
    ///
    /// The default value is **+3**, and the smallest accepted value is **+1** for normal updates,
    /// and **+2** for updates that contain packages from the "critical path".
    #[must_use]
    pub fn stable_karma(mut self, stable_karma: i32) -> Self {
        self.stable_karma = Some(stable_karma);
        self
    }

    /// method for optionally overriding the default unstable karma threshold
    ///
    /// The default value is **-3**. Updates that receive a total negative karma equal or smaller
    /// than this threshold are automatically retracted ("unpushed").
    #[must_use]
    pub fn unstable_karma(mut self, unstable_karma: i32) -> Self {
        self.unstable_karma = Some(unstable_karma);
        self
    }

    /// method for optionally specifying whether users should reboot or log out after installing
    /// this update
    ///
    /// If no value is specified, the server will create the update with a default value of
    /// [`UpdateSuggestion::Unspecified`].
    #[must_use]
    pub fn suggest(mut self, suggestion: UpdateSuggestion) -> Self {
        self.suggest = Some(suggestion);
        self
    }

    /// method for setting the optional list of associated gating test requirements
    ///
    /// The argument is expected to be a list of test names separated by spaces.
    #[must_use]
    pub fn requirements(mut self, requirements: &'a str) -> Self {
        self.requirements = Some(requirements);
        self
    }

    /// method for setting the optional preference whether feedback for associated bugs is
    /// necessary for positive karma to be counted against the total
    #[must_use]
    pub fn require_bugs(mut self, require_bugs: bool) -> Self {
        self.require_bugs = Some(require_bugs);
        self
    }

    /// method for setting the optional preference whether feedback for associated test cases is
    /// necessary for positive karma to be counted against the total
    #[must_use]
    pub fn require_testcases(mut self, require_testcases: bool) -> Self {
        self.require_testcases = Some(require_testcases);
        self
    }

    /// method for setting the optional preference whether an update should be pushed to stable
    /// after having been in the [`UpdateStatus::Testing`] state for at least `stable_days` days
    #[must_use]
    pub fn autotime(mut self, autotime: bool) -> Self {
        self.autotime = Some(autotime);
        self
    }

    /// method for optionally overriding the default stable days threshold
    ///
    /// The default value is **7 days**. The smallest accepted value is **7 days** for normal
    /// updates, **14 days** for updates that contain packages from the "critical path" or for
    /// EPEL updates, and **3 days** for updates that are submitted to pre-releases.
    #[must_use]
    pub fn stable_days(mut self, stable_days: u32) -> Self {
        self.stable_days = Some(stable_days);
        self
    }
}

impl<'a> SingleRequest<NewUpdate, NewUpdate> for UpdateCreator<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::POST
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/updates/"))
    }

    fn body(&self, csrf_token: Option<String>) -> Result<Option<String>, QueryError> {
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
                    Some(UpdateSeverity::Unspecified) => {
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
        };

        let bugs: Option<Vec<String>> = self
            .bugs
            .as_ref()
            .map(|bugs| bugs.iter().map(|b| format!("{b}")).collect());
        let bug_refs: Option<Vec<&str>> = bugs.as_ref().map(|b| b.iter().map(|s| s.as_str()).collect());

        let csrf_token = csrf_token.as_ref().unwrap_or_else(|| unreachable!());

        let new_update = match self.source {
            UpdateSource::Builds { builds } => UpdateData {
                builds: Some(builds),
                from_tag: None,
                bugs: bug_refs.as_deref(),
                display_name: self.display_name,
                close_bugs: self.close_bugs,
                update_type: self.update_type.unwrap_or(UpdateType::Unspecified),
                request: self.request,
                severity: self.severity,
                notes: self.notes,
                autokarma: self.autokarma,
                stable_karma: self.stable_karma,
                unstable_karma: self.unstable_karma,
                suggest: self.suggest,
                edited: None,
                requirements: self.requirements,
                require_bugs: self.require_bugs,
                require_testcases: self.require_testcases,
                autotime: self.autotime,
                stable_days: self.stable_days,
                csrf_token,
            },
            UpdateSource::Tag { tag } => UpdateData {
                builds: None,
                from_tag: Some(tag),
                bugs: bug_refs.as_deref(),
                display_name: self.display_name,
                close_bugs: self.close_bugs,
                update_type: self.update_type.unwrap_or(UpdateType::Unspecified),
                request: self.request,
                severity: self.severity,
                notes: self.notes,
                autokarma: self.autokarma,
                stable_karma: self.stable_karma,
                unstable_karma: self.unstable_karma,
                suggest: self.suggest,
                edited: None,
                requirements: self.requirements,
                require_bugs: self.require_bugs,
                require_testcases: self.require_testcases,
                autotime: self.autotime,
                stable_days: self.stable_days,
                csrf_token,
            },
        };

        Ok(Some(
            serde_json::to_string(&new_update).map_err(|error| QueryError::SerializationError { error })?,
        ))
    }

    fn parse(&self, string: &str) -> Result<NewUpdate, QueryError> {
        let new_update: NewUpdate = serde_json::from_str(string)?;
        Ok(new_update)
    }

    fn extract(&self, page: NewUpdate) -> NewUpdate {
        page
    }
}
