//! data type definitions for (de)serializing bodhi server requests and responses
//!
//! This module contains the data type definitions that are needed for deserializing bodhi server
//! responses, and for serializing bodhi server request data, which are also expored as public
//! API. Some internal wrapper types are implemented in the corresponding modules themselves.
//!
//! Additionally, the bodhi API accepts some arguments that are formally Strings, but only from a
//! limited set of valid values. Enumerations for all such types, plus (de)serialization support,
//! are provided in this module, as well.
//!
//! The bodhi server also returns date/time values in a non-standard format (neither RFC 3339 nor
//! ISO 8601 compliant). The [`BodhiDate`] wrapper type provides convenient parsing, printing, and
//! (de)serialization support for this format.
//!
//! FIXME mention [`FedoraRelease`] newtype wrapper around String with future-proof validated
//!   contents

mod dates;
pub use dates::*;

mod enums;
pub use enums::*;

mod error;
pub use error::InvalidValueError;

mod release;
pub use release::*;

mod schemas;
pub(crate) use schemas::*;

mod types;
pub use types::*;

// base URL of the fedora bodhi instance
pub const FEDORA_BODHI_URL: &str = "https://bodhi.fedoraproject.org";

// base URL of the fedora bodhi staging instance
pub const FEDORA_BODHI_STG_URL: &str = "https://bodhi.stg.fedoraproject.org";
