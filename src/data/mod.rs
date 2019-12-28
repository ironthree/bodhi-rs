//! This module contains the data types that are needed to deserialize bodhi server responses which
//! are also public outside this crate. Some internal data types for queries are implemented in the
//! corresponding query module.
//!
//! Additionally, the bodhi REST API has some arguments that accept Strings, but only from a limited
//! set of enumerated values. To abstract this, the corresponding query filters accept some of the
//! enum types defined here, instead of the String arguments directly.

mod dates;
pub use dates::*;

mod enums;
pub use enums::*;

mod schemas;
pub use schemas::*;

mod types;
pub use types::*;

/// base URL of the fedora bodhi instance
pub const FEDORA_BODHI_URL: &str = "https://bodhi.fedoraproject.org";

/// base URL of the fedora bodhi staging instance
pub const FEDORA_BODHI_STG_URL: &str = "https://bodhi.stg.fedoraproject.org";
