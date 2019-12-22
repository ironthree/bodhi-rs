#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::create::Create;
use crate::data::{Comment, Karma};
use crate::error::{BodhiError, QueryError};
use crate::query::{CSRFQuery, SinglePageQuery};
use crate::service::BodhiService;

// https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-1-POST
#[derive(Debug, Serialize)]
struct CommentData {
    /// alias of the update for which this is a comment
    update: String,
    /// comment text (default: `""`)
    text: Option<String>,
    /// comment karma (default: `0`)
    karma: Option<i32>,
    /// critpath karma (default: `0`)
    karma_critpath: Option<i32>,
    /// bug feedback vector (default: `[]`)
    bug_feedback: Option<Vec<i32>>,
    /// testcase feedback vector (default: `[]`)
    testcase_feedback: Option<Vec<i32>>,
    /// CSRF token
    csrf_token: String,
}

#[derive(Debug, Deserialize)]
pub struct NewComment {
    comment: Comment,
    caveats: Vec<String>,
}

#[derive(Debug)]
pub struct CommentBuilder {
    // TODO: take &Update instead
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
}

impl Create<NewComment> for CommentBuilder {
    fn create(&self, bodhi: &BodhiService) -> Result<NewComment, QueryError> {
        // TODO: check if lengths of feedback vectors is good
        let path = String::from("/comments/");

        let csrf_token = CSRFQuery::new().query(bodhi)?;

        let text = match &self.text {
            Some(text) => Some(text.to_owned()),
            None => None,
        };

        let karma: i32 = match &self.karma {
            Some(karma) => karma.clone().into(),
            None => 0,
        };

        let karma_critpath: Option<i32> = match &self.karma_critpath {
            Some(karma) => Some(karma.clone().into()),
            None => None,
        };

        let bug_feedback: Option<Vec<i32>> = match &self.bug_feedback {
            Some(feedback) => Some(feedback.iter().map(|k| k.clone().into()).collect()),
            None => None,
        };

        let testcase_feedback: Option<Vec<i32>> = match &self.testcase_feedback {
            Some(feedback) => Some(feedback.iter().map(|k| k.clone().into()).collect()),
            None => None,
        };

        let new_comment = CommentData {
            update: self.update.clone(),
            text,
            karma: Some(karma),
            karma_critpath,
            bug_feedback,
            testcase_feedback,
            csrf_token,
        };

        let data = match serde_json::to_string(&new_comment) {
            Ok(data) => data,
            Err(error) => return Err(QueryError::SerializationError { error }),
        };

        let response = bodhi.post(&path, data, None)?;
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
