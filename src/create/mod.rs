//! # wrappers for API calls that create new things
//!
//! This module contains data type definitions and request implementations related to creating
//! comments, overrides, and updates on a bodhi instance.

mod comments;
pub use comments::{BugFeedbackData, CommentCreator, NewComment, TestCaseFeedbackData};

mod overrides;
pub use overrides::{NewOverride, OverrideCreator};

mod updates;
pub use updates::{NewUpdate, UpdateCreator};
