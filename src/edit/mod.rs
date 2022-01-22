//! # wrappers for API calls that edit existing things
//!
//! This module contains data type definitions and request implementations related to editing
//! overrides and updates on a bodhi instance.

mod overrides;
pub use overrides::{EditedOverride, OverrideEditor};

mod updates;
pub use updates::{EditedUpdate, UpdateEditor, UpdateStatusRequester, UpdateTestResultWaiver};
