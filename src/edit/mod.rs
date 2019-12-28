//! This module contains implementations for editing **Overrides** and **Updates** on a bodhi
//! instance. Editing **Releases** is possible with the REST API, but not implemented yet.

mod traits;
pub(crate) use traits::Edit;

mod overrides;
pub use overrides::{EditedOverride, OverrideEditor};

mod updates;
pub use updates::{EditedUpdate, UpdateEditor, UpdateStatusRequester, UpdateTestResultWaiver};
