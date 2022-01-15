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

#[derive(Debug, Serialize)]
struct BugFeedbackData {
    bug_id: u32,
    karma: Karma,
}

#[derive(Debug, Serialize)]
struct TestCaseFeedbackData<'a> {
    testcase_name: &'a str,
    karma: Karma,
}

// This struct contains the values that are returned when creating a new comment.
#[derive(Debug, Deserialize)]
pub struct NewComment {
    // the newly created comment
    pub comment: Comment,
    // additional server messages
    pub caveats: Vec<HashMap<String, String>>,
}

// This struct contains all the values that are necessary for creating a new comment. Methods to
// supply optional arguments are also available.
#[derive(Debug)]
pub struct CommentCreator<'a> {
    update: &'a str,
    text: Option<&'a str>,
    karma: Option<Karma>,
    bug_feedback: Option<Vec<BugFeedbackData>>,
    testcase_feedback: Option<Vec<TestCaseFeedbackData<'a>>>,
}

impl<'a> CommentCreator<'a> {
    // This method has to be used to create and initialize a new `CommentBuilder`.
    pub fn new(update: &'a str) -> Self {
        CommentCreator {
            update,
            text: None,
            karma: None,
            bug_feedback: None,
            testcase_feedback: None,
        }
    }

    // Add optional text to the comment.
    #[must_use]
    pub fn text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }

    // Add optional general karma feedback to the comment.
    #[must_use]
    pub fn karma(mut self, karma: Karma) -> Self {
        self.karma = Some(karma);
        self
    }

    // Add optional bug feedback to the comment.
    //
    // If the specified bug is not associated with the update, this is discarded server-side.
    #[must_use]
    pub fn bug_feedback(mut self, bug_id: u32, karma: Karma) -> Self {
        let feedback = BugFeedbackData { bug_id, karma };

        match &mut self.bug_feedback {
            Some(bug_feedback) => bug_feedback.push(feedback),
            None => self.bug_feedback = Some(vec![feedback]),
        }

        self
    }

    // Add optional test case feedback to the comment.
    //
    // If the specified test case is not associated with the update, this is discarded server-side.
    #[must_use]
    pub fn testcase_feedback(mut self, testcase_name: &'a str, karma: Karma) -> Self {
        let feedback = TestCaseFeedbackData { karma, testcase_name };

        match &mut self.testcase_feedback {
            Some(testcase_feedback) => testcase_feedback.push(feedback),
            None => self.testcase_feedback = Some(vec![feedback]),
        }

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
            text: match self.text {
                Some(text) => Some(text),
                None => None,
            },
            karma: match self.karma {
                Some(karma) => karma,
                None => Karma::Neutral,
            },
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
    // This method creates a new `CommentBuilder` for commenting on this `Update`.
    pub fn comment(&self) -> CommentCreator {
        CommentCreator::new(self.alias.as_str())
    }
}
