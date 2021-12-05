// ! This module contains all the REST API query wrappers that attempt to map the REST-y API to an
// ! idiomatic Rust API, using builder patterns to construct complex queries.
// !
// ! For example, the bodhi REST API accepts `Sequence` arguments for certain keywords, which have
// to ! be encoded as comma-separated lists of Strings. The Rust API abstracts this as with methods
// on ! the query that accept normal Strings and then construct the comma-separated lists on demand.

pub mod builds;
pub use builds::{BuildNVRQuery, BuildQuery};

pub mod comments;
pub use comments::{CommentIDQuery, CommentQuery};

//pub mod composes;
//pub use composes::{ComposeQuery, ComposeReleaseRequestQuery};

pub mod csrf;
pub use csrf::CSRFQuery;

//pub mod overrides;
//pub use overrides::{OverrideNVRQuery, OverrideQuery};

//pub mod packages;
//pub use packages::PackageQuery;

//pub mod releases;
//pub use releases::{ReleaseNameQuery, ReleaseQuery};

//pub mod updates;
//pub use updates::{UpdateIDQuery, UpdateQuery};

//pub mod users;
//pub use users::{UserNameQuery, UserQuery};

//pub(crate) mod traits;
//pub(crate) use traits::*;
