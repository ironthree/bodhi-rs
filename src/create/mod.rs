use serde::{Deserialize, Serialize};

use crate::data::Karma;
use crate::error::{BodhiError, QueryError};
use crate::query::csrf::CSRFQuery;
use crate::service::BodhiService;

#[derive(Debug, Serialize)]
struct CommentData {
    update: String,
    text: String,
    karma: i32,
    karma_critpath: Option<i32>,
    bug_feedback: Option<Vec<i32>>,
    testcase_feedback: Option<Vec<i32>>,
    csrf_token: String,
}

#[derive(Debug, Deserialize)]
pub struct NewComment {
    comment: u32,
    caveats: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct CommentBuilder {
    update: String,
    text: Option<String>,
    karma: Option<Karma>,
    karma_critpath: Option<Karma>,
    bug_feedback: Option<Vec<Karma>>,
    testcase_feedback: Option<Vec<Karma>>,
}

impl CommentBuilder {
    pub fn new(update: String) -> Self {
        CommentBuilder {
            update,
            text: None,
            karma: None,
            karma_critpath: None,
            bug_feedback: None,
            testcase_feedback: None,
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn karma(mut self, karma: Karma) -> Self {
        self.karma = Some(karma);
        self
    }

    pub fn karma_critpath(mut self, karma: Karma) -> Self {
        self.karma_critpath = Some(karma);
        self
    }

    pub fn bug_feedback(mut self, feedback: Vec<Karma>) -> Self {
        self.bug_feedback = Some(feedback);
        self
    }

    pub fn testcase_feedback(mut self, feedback: Vec<Karma>) -> Self {
        self.testcase_feedback = Some(feedback);
        self
    }

    pub fn create(self, bodhi: &BodhiService) -> Result<NewComment, QueryError> {
        // let user = bodhi.username()?;
        bodhi.username()?;

        // TODO: check if lengths of feedback vectors is good
        let path = String::from("/comments/");

        let csrf_token = CSRFQuery::new().query(&bodhi)?;

        let text = match self.text {
            Some(text) => text,
            None => String::from(""),
        };

        let karma: i32 = match self.karma {
            Some(karma) => karma.into(),
            None => 0,
        };

        let karma_critpath: Option<i32> = match self.karma_critpath {
            Some(karma) => Some(karma.into()),
            None => None,
        };

        let bug_feedback: Option<Vec<i32>> = match self.bug_feedback {
            Some(feedback) => Some(feedback.into_iter().map(|k| k.into()).collect()),
            None => None,
        };

        let testcase_feedback: Option<Vec<i32>> = match self.testcase_feedback {
            Some(feedback) => Some(feedback.into_iter().map(|k| k.into()).collect()),
            None => None,
        };

        let new_comment = CommentData {
            update: self.update,
            text,
            karma,
            karma_critpath,
            bug_feedback,
            testcase_feedback,
            csrf_token,
        };

        let data = match serde_json::to_string(&new_comment) {
            Ok(data) => data,
            Err(error) => return Err(QueryError::SerializationError { error }),
        };

        // FIXME

        let mut response = bodhi.post(&path, data, None)?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().unwrap_or_else(|_| String::from(""));

            let error: BodhiError = serde_json::from_str(&text)?;
            return Err(QueryError::BodhiError { error });
        };

        let result = response.text()?;
        let comment: NewComment = serde_json::from_str(&result)?;

        Ok(comment)
    }
}

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

    pub fn create(self, bodhi: &BodhiService) -> Result<NewOverride, QueryError> {
        // let user = bodhi.username()?;
        bodhi.username()?;

        let path = String::from("/overrides/");

        let csrf_token = CSRFQuery::new().query(&bodhi)?;

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

        // FIXME

        let mut response = bodhi.post(&path, data, None)?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().unwrap_or_else(|_| String::from(""));

            let error: BodhiError = serde_json::from_str(&text)?;
            return Err(QueryError::BodhiError { error });
        };

        let result = response.text()?;
        let new_override: NewOverride = serde_json::from_str(&result)?;

        Ok(new_override)
    }
}