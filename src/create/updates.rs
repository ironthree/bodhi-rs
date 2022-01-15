use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{Update, UpdateData, UpdateRequest, UpdateSeverity, UpdateSuggestion, UpdateType};
use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

// This struct contains the values that are returned when creating a new update.
#[derive(Debug, Deserialize)]
pub struct NewUpdate {
    // the newly created update
    #[serde(flatten)]
    pub update: Update,
    // additional server messages
    pub caveats: Vec<HashMap<String, String>>,
}

#[derive(Debug)]
enum UpdateSource<'a> {
    Builds { builds: &'a [&'a str] },
    Tag { tag: &'a str },
}

// This struct contains all the values that are necessary for creating a new update. Methods to
// supply optional arguments are also available.
#[derive(Debug)]
pub struct UpdateCreator<'a> {
    // mandatory fields
    source: UpdateSource<'a>,
    notes: &'a str,

    // optional fields
    bugs: Option<Vec<u32>>,
    display_name: Option<String>,
    close_bugs: Option<bool>,
    update_type: Option<UpdateType>,
    request: Option<UpdateRequest>,
    severity: Option<UpdateSeverity>,
    autokarma: Option<bool>,
    stable_karma: Option<i32>,
    unstable_karma: Option<i32>,
    suggest: Option<UpdateSuggestion>,
    requirements: Option<String>,
    require_bugs: Option<bool>,
    require_testcases: Option<bool>,
    autotime: Option<bool>,
    stable_days: Option<u32>,
}

impl<'a> UpdateCreator<'a> {
    // Use this method when creating an update for a list of builds.
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

    // Use this method when creating an update for a side tag.
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

    // Add related bug(s) to the update.
    #[must_use]
    pub fn bugs(mut self, bugs: Vec<u32>) -> Self {
        self.bugs = Some(bugs);
        self
    }

    // Set the flag whether bugs will be closed when the update is pushed to stable.
    #[must_use]
    pub fn close_bugs(mut self, close_bugs: bool) -> Self {
        self.close_bugs = Some(close_bugs);
        self
    }

    // Add a custom user-visible title to the update.
    #[must_use]
    pub fn display_name(mut self, display_name: String) -> Self {
        self.display_name = Some(display_name);
        self
    }

    // Flag to specify the type of update (new package, bug fix, enhancement, security update, or
    // unspecified). For security updates, the severity also has to be specified.
    #[must_use]
    pub fn update_type(mut self, update_type: UpdateType) -> Self {
        self.update_type = Some(update_type);
        self
    }

    // Flag to specify the update severity (primarily used for security updates, where this flag is
    // mandatory).
    #[must_use]
    pub fn severity(mut self, severity: UpdateSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    // Set the flag whether the update can automatically be pushed to stable once it reaches the
    // specified stable karma.
    #[must_use]
    pub fn autokarma(mut self, autokarma: bool) -> Self {
        self.autokarma = Some(autokarma);
        self
    }

    // Manually set the stable karma feedback threshold.
    #[must_use]
    pub fn stable_karma(mut self, stable_karma: i32) -> Self {
        self.stable_karma = Some(stable_karma);
        self
    }

    // Manually set the unstable karma feedback threshold.
    #[must_use]
    pub fn unstable_karma(mut self, unstable_karma: i32) -> Self {
        self.unstable_karma = Some(unstable_karma);
        self
    }

    // Flag to specify whether users should log out or reboot to successfully apply an update.
    #[must_use]
    pub fn suggest(mut self, suggestion: UpdateSuggestion) -> Self {
        self.suggest = Some(suggestion);
        self
    }

    // Add custom taskotron requirements.
    #[must_use]
    pub fn requirements(mut self, requirements: String) -> Self {
        self.requirements = Some(requirements);
        self
    }

    // Flag to indicate whether bug feedback is required for karma to be counted.
    #[must_use]
    pub fn require_bugs(mut self, require_bugs: bool) -> Self {
        self.require_bugs = Some(require_bugs);
        self
    }

    // Flag to indicate whether test case feedback is required for karma to be counted.
    #[must_use]
    pub fn require_testcases(mut self, require_testcases: bool) -> Self {
        self.require_testcases = Some(require_testcases);
        self
    }

    // Set the flag whether the update can automatically be pushed to stable once it reaches the
    // specified days in testing.
    #[must_use]
    pub fn autotime(mut self, autotime: bool) -> Self {
        self.autotime = Some(autotime);
        self
    }

    // Manually specify the minimum duration the update has to stay in testing.
    //
    // The default is 7 days for stable updates, 14 days for stable updates containing critpath
    // packages, and 3 days for fedora pre-releases.
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
        };

        let bugs: Option<Vec<String>> = self
            .bugs
            .as_ref()
            .map(|bugs| bugs.iter().map(|b| format!("{}", b)).collect());

        let csrf_token = csrf_token.as_ref().unwrap_or_else(|| unreachable!());

        let new_update = match self.source {
            UpdateSource::Builds { builds } => UpdateData {
                builds: Some(builds),
                from_tag: None,
                bugs: bugs.as_ref(),
                display_name: match &self.display_name {
                    Some(string) => Some(string),
                    None => None,
                },
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
                edited: None,
                requirements: match &self.requirements {
                    Some(string) => Some(string),
                    None => None,
                },
                require_bugs: self.require_bugs,
                require_testcases: self.require_testcases,
                autotime: self.autotime,
                stable_days: self.stable_days,
                csrf_token,
            },
            UpdateSource::Tag { tag } => UpdateData {
                builds: None,
                from_tag: Some(tag),
                bugs: bugs.as_ref(),
                display_name: match &self.display_name {
                    Some(string) => Some(string),
                    None => None,
                },
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
                edited: None,
                requirements: match &self.requirements {
                    Some(string) => Some(string),
                    None => None,
                },
                require_bugs: self.require_bugs,
                require_testcases: self.require_testcases,
                autotime: self.autotime,
                stable_days: self.stable_days,
                csrf_token,
            },
        };

        match serde_json::to_string(&new_update) {
            Ok(result) => Ok(Some(result)),
            Err(error) => Err(QueryError::SerializationError { error }),
        }
    }

    fn parse(&self, string: &str) -> Result<NewUpdate, QueryError> {
        let new_update: NewUpdate = serde_json::from_str(string)?;
        Ok(new_update)
    }

    fn extract(&self, page: NewUpdate) -> NewUpdate {
        page
    }
}
