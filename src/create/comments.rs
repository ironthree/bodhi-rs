use serde::{Deserialize, Serialize};

use crate::error::{BodhiError, QueryError};
use crate::{BodhiService, CSRFQuery, Comment, Create, Karma, SinglePageQuery, Update};

// API documentation: <https://bodhi.fedoraproject.org/docs/server_api/rest/comments.html#service-1-POST>
#[derive(Debug, Serialize)]
struct CommentData<'a> {
    update: &'a str,
    text: Option<&'a str>,
    karma: Option<&'a Karma>,
    karma_critpath: Option<&'a Karma>,
    bug_feedback: Option<&'a Vec<BugFeedbackData>>,
    testcase_feedback: Option<&'a Vec<TestCaseFeedbackData<'a>>>,
    csrf_token: &'a str,
}

#[derive(Debug, Serialize)]
struct BugFeedbackData {
    bug_id: u32,
    karma: Karma,
}

#[derive(Debug, Serialize)]
struct TestCaseFeedbackData<'a> {
    karma: Karma,
    testcase_name: &'a str,
}

/// This struct contains the values that are returned when creating a new comment.
#[derive(Debug, Deserialize)]
pub struct NewComment {
    /// the newly created comment
    pub comment: Comment,
    /// additional server messages
    pub caveats: Vec<String>,
}

/// This struct contains all the values that are necessary for creating a new comment. Methods to
/// supply optional arguments are also available.
#[derive(Debug)]
pub struct CommentBuilder<'a> {
    update: &'a str,
    text: Option<&'a str>,
    karma: Option<Karma>,
    karma_critpath: Option<Karma>,
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
            karma_critpath: None,
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

    /// Add optional critpath karma feedback to the comment.
    pub fn karma_critpath(mut self, karma: Karma) -> Self {
        self.karma_critpath = Some(karma);
        self
    }

    /// Add optional bug feedback to the comment.
    pub fn bug_feedback(mut self, bug_id: u32, karma: Karma) -> Self {
        let feedback = BugFeedbackData { bug_id, karma };

        match &mut self.bug_feedback {
            Some(bug_feedback) => bug_feedback.push(feedback),
            None => self.bug_feedback = Some(vec![feedback]),
        }

        self
    }

    /// Add optional test case feedback to the comment.
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

        let new_comment = CommentData {
            update: &self.update,
            text: match &self.text {
                Some(text) => Some(&text),
                None => None,
            },
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
