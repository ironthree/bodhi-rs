//! # data type definitions for (de)serializing bodhi server requests and responses
//!
//! This module contains the data type definitions that are needed for deserializing bodhi server
//! responses, and for serializing bodhi server request data, which are also expored as public
//! API. Some internal wrapper types are implemented in the corresponding modules themselves.
//!
//! Additionally, there is [`serde`] support for (de)serializing all data types that are either
//! returned as JSON-encoded responses, or expected as JSON-encoded parameters for `POST` requests.
//!
//! ## structured data types
//!
//! Struct definitions are included for all structured data types known to bodhi. These correspond
//! as closely as possible to the python class definitions of the server and bodhi client
//! implementations.
//!
//! ## enumerated string types
//!
//! Some fields in structured JSON data are strings, but there is only a limited number of values
//! that are actually valid identifiers. This crate includes definitions of enumerated types in such
//! a way that their variants correspond to valid strings. This makes it impossible to construct
//! invalid values for those fields, and they are also turned into enum variants when they are
//! deserialized from JSON.
//!
//! ## custom datetime type
//!
//! The bodhi server also returns date/time values in a non-standard format (neither RFC 3339 nor
//! ISO 8601 compliant). The [`BodhiDate`] wrapper type provides convenient parsing, printing, and
//! (de)serialization support for this format.
//!
//! ## custom release type
//!
//! The release identifiers for Fedora / EPEL releases are treated in a different way. They are
//! formatted as strings, but obviously not all strings are valid release identifiers. The
//! [`FedoraRelease`] type is implemented as a newtype wrapper around strings, but all methods which
//! construct instances of the type validate their input, to ensure only correctly formatted
//! release identifiers can be constructed.
//!
//! Previous releases of this crate defined [`FedoraRelease`] as an enum with variants for all
//! supported Fedora and EPEL releases. This proved to be hard to maintain, because the list of
//! valid identifiers changes every few months, which made it necessary to constantly add new enum
//! variants and recompile all programs that used this crate.
//!
//! By using a newtype wrapper around strings, it is no longer necessary to add new enum variants
//! for new releases, but release values are still validated against the expected format of Fedora
//! and EPEL release identifiers.

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
pub(crate) const FEDORA_BODHI_URL: &str = "https://bodhi.fedoraproject.org";

// base URL of the fedora bodhi staging instance
pub(crate) const FEDORA_BODHI_STG_URL: &str = "https://bodhi.stg.fedoraproject.org";
