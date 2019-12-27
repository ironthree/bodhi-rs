#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::error::{BodhiError, QueryError};
use crate::{BodhiService, CSRFQuery, Comment, Create, Karma, SinglePageQuery};

/// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-1-POST>
#[derive(Debug, Serialize)]
struct CommentData<'a> {
    /// alias of the update for which this is a comment
    update: &'a String,
    /// comment text (default: `""`)
    text: Option<&'a String>,
    /// comment karma (default: `0`)
    karma: Option<&'a Karma>,
    /// critpath karma (default: `0`)
    karma_critpath: Option<&'a Karma>,
    /// bug feedback vector (default: `[]`)
    bug_feedback: Option<&'a Vec<Karma>>,
    /// testcase feedback vector (default: `[]`)
    testcase_feedback: Option<&'a Vec<Karma>>,
    /// CSRF token
    csrf_token: &'a String,
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

        let new_comment = CommentData {
            update: &self.update,
            text: self.text.as_ref(),
            karma: self.karma.as_ref(),
            karma_critpath: self.karma_critpath.as_ref(),
            bug_feedback: self.bug_feedback.as_ref(),
            testcase_feedback: self.testcase_feedback.as_ref(),
            csrf_token: &csrf_token,
        };

        let data = match serde_json::to_string(&new_comment) {
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
        let comment: NewComment = serde_json::from_str(&result)?;

        Ok(comment)
    }
}
