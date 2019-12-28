//! This module contains implementations for creating **Comments**, **Overrides**, and **Updates**
//! on a bodhi instance. Creating **Releases** is possible with the REST API, but not implemented
//! yet.

mod traits;
pub(crate) use traits::Create;

mod comments;
pub use comments::{CommentBuilder, NewComment};

mod overrides;
pub use overrides::{NewOverride, OverrideBuilder};

mod updates;
pub use updates::{NewUpdate, UpdateBuilder};
