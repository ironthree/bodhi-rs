// ! This module contains implementations for creating **Comments**, **Overrides**, and **Updates**
// ! on a bodhi instance. Creating **Releases** is possible with the REST API, but not implemented
// ! yet.

mod comments;
pub use comments::{CommentCreator, NewComment};

mod overrides;
pub use overrides::{NewOverride, OverrideCreator};

mod updates;
pub use updates::{NewUpdate, UpdateCreator};
