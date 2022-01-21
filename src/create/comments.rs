use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::{Comment, Karma, Update};
use crate::error::QueryError;
use crate::request::{RequestMethod, SingleRequest};

// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-1-POST>
#[derive(Debug, Serialize)]
struct CommentData<'a> {
    update: &'a str,
    text: Option<&'a str>,
    karma: Karma,
    csrf_token: &'a str,

    #[serde(flatten)]
    feedback: HashMap<String, String>,
}

/// data type for bug feedback
#[derive(Debug, Serialize)]
pub struct BugFeedbackData {
    bug_id: u32,
    karma: Karma,
}

impl BugFeedbackData {
    /// constructor for [`BugFeedbackData`] with arguments for both mandatory parameters
    pub fn new(bug_id: u32, karma: Karma) -> Self {
        BugFeedbackData { bug_id, karma }
    }
}

/// data type for test case feedback
#[derive(Debug, Serialize)]
pub struct TestCaseFeedbackData<'a> {
    testcase_name: &'a str,
    karma: Karma,
}

impl<'a> TestCaseFeedbackData<'a> {
    /// constructor for [`TestCaseFeedBackData`] with arguments for both mandatory parameters
    pub fn new(testcase_name: &'a str, karma: Karma) -> Self {
        TestCaseFeedbackData { testcase_name, karma }
    }
}

/// data of this type is returned after successfully posting a new comment
#[derive(Debug, Deserialize)]
pub struct NewComment {
    /// new comment that was just created
    pub comment: Comment,
    /// additional server messages
    pub caveats: Vec<HashMap<String, String>>,
}

/// data type wrapping all mandatory and optional parameters for creating a new comment
#[derive(Debug)]
pub struct CommentCreator<'a> {
    update: &'a str,
    text: Option<&'a str>,
    karma: Option<Karma>,
    bug_feedback: Option<&'a [BugFeedbackData]>,
    testcase_feedback: Option<&'a [TestCaseFeedbackData<'a>]>,
}

impl<'a> CommentCreator<'a> {
    /// constructor for [`CommentCreator`] with default values for optional parameters
    pub fn new(update: &'a str) -> Self {
        CommentCreator {
            update,
            text: None,
            karma: None,
            bug_feedback: None,
            testcase_feedback: None,
        }
    }

    /// method for setting optional comment text
    #[must_use]
    pub fn text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }

    /// method for setting optional karma value
    #[must_use]
    pub fn karma(mut self, karma: Karma) -> Self {
        self.karma = Some(karma);
        self
    }

    /// method for adding optional bug feedback
    ///
    /// Any bug IDs that do not match bug IDs associated with the update this comment is posted for
    /// are discarded by the server.
    #[must_use]
    pub fn bug_feedback(mut self, feedbacks: &'a [BugFeedbackData]) -> Self {
        self.bug_feedback = Some(feedbacks);
        self
    }

    /// method for adding optional test case feedback
    ///
    /// Any test cases that do not match test cases associated with the update this comment is
    /// posted for are discarded by the server.
    #[must_use]
    pub fn testcase_feedback(mut self, feedbacks: &'a [TestCaseFeedbackData<'a>]) -> Self {
        self.testcase_feedback = Some(feedbacks);
        self
    }
}

impl<'a> SingleRequest<NewComment, NewComment> for CommentCreator<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::POST
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/comments/"))
    }

    fn body(&self, csrf_token: Option<String>) -> Result<Option<String>, QueryError> {
        let mut feedback: HashMap<String, String> = HashMap::new();

        let karma_string = |k: Karma| match k {
            Karma::Positive => String::from("1"),
            Karma::Neutral => String::from("0"),
            Karma::Negative => String::from("-1"),
        };

        // bug and testcase feedback is expected in a really weird format, see:
        // https://github.com/fedora-infra/bodhi/issues/3888#issuecomment-577793271
        if let Some(items) = &self.bug_feedback {
            for (pos, item) in items.iter().enumerate() {
                feedback.insert(format!("bug_feedback.{}.bug_id", pos), item.bug_id.to_string());
                feedback.insert(format!("bug_feedback.{}.karma", pos), karma_string(item.karma));
            }
        };

        if let Some(items) = &self.testcase_feedback {
            for (pos, item) in items.iter().enumerate() {
                feedback.insert(
                    format!("testcase_feedback.{}.testcase_name", pos),
                    item.testcase_name.to_string(),
                );
                feedback.insert(format!("testcase_feedback.{}.karma", pos), karma_string(item.karma));
            }
        };

        let new_comment = CommentData {
            update: self.update,
            text: self.text,
            karma: self.karma.unwrap_or(Karma::Neutral),
            feedback,
            csrf_token: csrf_token.as_ref().unwrap_or_else(|| unreachable!()),
        };

        match serde_json::to_string(&new_comment) {
            Ok(result) => Ok(Some(result)),
            Err(error) => Err(QueryError::SerializationError { error }),
        }
    }

    fn parse(&self, string: &str) -> Result<NewComment, QueryError> {
        let new_comment: NewComment = serde_json::from_str(string)?;
        Ok(new_comment)
    }

    fn extract(&self, page: NewComment) -> NewComment {
        page
    }
}

impl Update {
    /// constructor for [`CommentCreator`] which takes the update ID from an existing update
    pub fn comment(&self) -> CommentCreator {
        CommentCreator::new(self.alias.as_str())
    }
}
