//! This library provides a Rust-y wrapper around the bodhi REST API.
//!
//! [bodhi] is the web service for managing updates for fedora-based linux
//! distributions. It provides a REST API for querying its database, and
//! for creating new updates, comments, etc.
//!
//! [bodhi]: https://github.com/fedora-infra/bodhi
//!
//! The library is structured like this:
//!
//! - `BodhiService`, which contains all information related to connecting
//!   to a remote bodhi instance
//! - a set of `*Query` structs and implementations, which wrap the REST API
//!   with a Rust-y API
//! - data type definitions, used for deserializing JSON responses with [serde]
//!
//! [serde]: https://docs.rs/serde
//!
//! ## Data type definitions
//!
//! The data type definitions used for deserializing the server JSON responses
//! are contained in the `data` module. Some definitions are used only
//! internally (for example, for deserializing paginated results), and are not
//! publicly exported. They are located next to the `Query` they are used for.
//!
//! ## Rust API Convenience
//!
//! For convenience, some enumerated types that are just strings in the actual
//! REST API are wrapped as proper `enum` types, and queries that return
//! paginated results are not exposed to users of this library, but handled
//! completely internally to return a union of all result pages.
//!
//! **NOTE**: Some `Query` modifiers can be called multiple times (methods
//! marked with plural names), and other filters are mutually exclusive and
//! should only be called *once* on a query (methods marked with singular name).
//! If a filter that only allows one argument is called more than once, the last
//! supplied argument will override arguments that were supplied to previous
//! method calls.
//!
//! ## Usage
//!
//! To query a remote bodhi instance, first construct a `BodhiService` instance
//! with the desired properties (server URL, request timeout, retry limit).
//!
//! Then, construct the required queries, and run the query against the
//! `BodhiService` instance. In theory, this would let you run the same query
//! multiple times, possibly against different server instances or with
//! different connection settings:
//!
//! ```
//! let bodhi = bodhi::BodhiServiceBuilder::default().build().unwrap();
//!
//! let package_query = bodhi::query::PackageQuery::new().name("rust".to_string());
//! let packages = package_query.query(&bodhi).unwrap();
//! ```

pub mod data;
pub use data::{FEDORA_BODHI_STG_URL, FEDORA_BODHI_URL};

// pub mod client;

pub mod service;
pub use service::{BodhiService, BodhiServiceBuilder};

pub mod error;

pub mod create;
pub mod edit;
pub mod query;

#[cfg(test)]
mod tests;
