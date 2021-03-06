//! This library provides a Rust-y wrapper around the bodhi REST API.
//!
//! [bodhi] is the web service for managing updates for fedora-based linux distributions. It
//! provides a REST API for querying its database, and for creating new updates, comments, etc.
//!
//! [bodhi]: https://github.com/fedora-infra/bodhi
//!
//! The library is structured like this:
//!
//! - [`BodhiService`](service/struct.BodhiService.html), which contains all information related to
//!   connecting to a remote bodhi instance
//! - a set of `*Query` structs and implementations for querying bodhi, which wrap the REST API with
//!   a Rust-y API
//! - a set of `Create` implementations for creating new data on bodhi
//! - a set of `Edit` implementations for editing data on bodhi
//! - data type and enum definitions, used for (de)serializing JSON values with [serde]
//!
//! [serde]: https://docs.rs/serde
//!
//! ## Data type definitions
//!
//! The data type definitions used for deserializing the server JSON responses are contained in the
//! `data` module. Some definitions are used only internally (for example, for deserializing
//! paginated results), and are not publicly exported. They are located next to the `Query` they are
//! used for.
//!
//! ## Rust API Convenience
//!
//! For convenience, some enumerated types that are just strings in the actual REST API are wrapped
//! as proper `enum` types, and queries that return paginated results are not exposed to users of
//! this library, but handled completely internally to return a union of all result pages.
//!
//! ## Usage
//!
//! To query a remote bodhi instance, first construct a
//! [`BodhiService`](service/struct.BodhiService.html) instance with the desired properties (server
//! URL, request timeout, retry limit).
//!
//! Then, construct the required queries, and run the query against the
//! [`BodhiService`](service/struct.BodhiService.html) instance. In theory, this would let you run
//! the same query multiple times, possibly against different server instances or with different
//! connection settings:
//!
//! ```
//! let bodhi = bodhi::BodhiServiceBuilder::default().build().unwrap();
//!
//! let package_query = bodhi::PackageQuery::new().name("rust");
//!
//! # #[cfg(feature = "online-tests")]
//! let packages = bodhi.query(package_query).unwrap();
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

pub mod data;
pub use data::*;

pub mod service;
pub use service::{BodhiService, BodhiServiceBuilder};

pub mod error;

pub mod create;
pub use create::*;

pub mod edit;
pub use edit::*;

pub mod query;
pub use query::*;

#[cfg(test)]
mod tests;
