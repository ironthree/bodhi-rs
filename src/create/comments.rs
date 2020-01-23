use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{BodhiError, QueryError};
use crate::{BodhiService, CSRFQuery, Comment, Create, Karma, SinglePageQuery, Update};

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

/// This struct contains the values that are returned when creating a new comment.
#[derive(Debug, Deserialize)]
pub struct NewComment {
    /// the newly created comment
    pub comment: Comment,
    /// additional server messages
    pub caveats: Vec<HashMap<String, String>>,
}

/// This struct contains all the values that are necessary for creating a new comment. Methods to
/// supply optional arguments are also available.
#[derive(Debug)]
pub struct CommentBuilder<'a> {
    update: &'a str,
    text: Option<&'a str>,
    karma: Option<Karma>,
    bug_feedback: Option<Vec<BugFeedbackData>>,
    testcase_feedback: Option<Vec<TestCaseFeedbackData<'a>>>,
}

impl<'a> CommentBuilder<'a> {
    /// This method has to be used to create and initialize a new `CommentBuilder`.
    pub fn new(update: &'a str) -> Self {
        CommentBuilder {
            update,
            text: None,
            karma: None,
            bug_feedback: None,
            testcase_feedback: None,
        }
    }

    /// Add optional text to the comment.
    pub fn text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }

    /// Add optional general karma feedback to the comment.
    pub fn karma(mut self, karma: Karma) -> Self {
        self.karma = Some(karma);
        self
    }

    /// Add optional bug feedback to the comment.
    ///
    /// If the specified bug is not associated with the update, this is discarded server-side.
    pub fn bug_feedback(mut self, bug_id: u32, karma: Karma) -> Self {
        let feedback = BugFeedbackData { bug_id, karma };

        match &mut self.bug_feedback {
            Some(bug_feedback) => bug_feedback.push(feedback),
            None => self.bug_feedback = Some(vec![feedback]),
        }

        self
    }

    /// Add optional test case feedback to the comment.
    ///
    /// If the specified test case is not associated with the update, this is discarded server-side.
    pub fn testcase_feedback(mut self, testcase_name: &'a str, karma: Karma) -> Self {
        let feedback = TestCaseFeedbackData { karma, testcase_name };

        match &mut self.testcase_feedback {
            Some(testcase_feedback) => testcase_feedback.push(feedback),
            None => self.testcase_feedback = Some(vec![feedback]),
        }

        self
    }
}

impl<'a> Create<NewComment> for CommentBuilder<'a> {
    fn create(&self, bodhi: &BodhiService) -> Result<NewComment, QueryError> {
        let path = String::from("/comments/");

        let csrf_token = CSRFQuery::new().query(bodhi)?;

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
            update: &self.update,
            text: match &self.text {
                Some(text) => Some(&text),
                None => None,
            },
            karma: match self.karma {
                Some(karma) => karma,
                None => Karma::Neutral,
            },
            feedback,
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
        let new_comment: NewComment = serde_json::from_str(&result)?;

        Ok(new_comment)
    }
}

impl Update {
    /// This method creates a new `CommentBuilder` for commenting on this `Update`.
    pub fn comment(&self) -> CommentBuilder {
        CommentBuilder::new(self.alias.as_str())
    }
}
