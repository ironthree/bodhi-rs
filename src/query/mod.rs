// ! This module contains all the REST API query wrappers that attempt to map the REST-y API to an
// ! idiomatic Rust API, using builder patterns to construct complex queries.
// !
// ! For example, the bodhi REST API accepts `Sequence` arguments for certain keywords, which have
// ! to be encoded as comma-separated lists of Strings. The Rust API abstracts this as with methods
// ! on the query that accept normal Strings and then construct the comma-separated lists on demand.

mod builds;
pub use builds::{BuildNVRQuery, BuildPageQuery, BuildQuery};

mod comments;
pub use comments::{CommentIDQuery, CommentPageQuery, CommentQuery};

mod composes;
pub use composes::{ComposeQuery, ComposeReleaseRequestQuery};

mod csrf;
pub use csrf::CSRFQuery;

mod overrides;
pub use overrides::{OverrideNVRQuery, OverridePageQuery, OverrideQuery};

mod packages;
pub use packages::{PackagePageQuery, PackageQuery};

mod releases;
pub use releases::{ReleaseNameQuery, ReleasePageQuery, ReleaseQuery};

mod updates;
pub use updates::{UpdateIDQuery, UpdatePageQuery, UpdateQuery};

mod users;
pub use users::{UserNameQuery, UserPageQuery, UserQuery};
