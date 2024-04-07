//! # bodhi-rs
//!
//! This crate provides an opinionated, idiomatic, and async-aware Rust wrapper around the REST API
//! that is provided by the [bodhi] web service.
//!
//! [bodhi]: https://github.com/fedora-infra/bodhi
//!
//! It includes built-in support for both instances operated by the Fedora Project ([production]
//! and [staging]), but also allows specifying custom base URLs for API calls for custom deployments
//! or for testing purposes.
//!
//! [production]: https://bodhi.fedoraproject.org
//! [staging]: https://bodhi.stg.fedoraproject.org
//!
//! Almost all public items in this crate are re-exported in the crate's root namespace for
//! convenience, but documentation for public items from submodules is linked from the module where
//! they are defined. Documentation for all public modules is available [below](crate#modules).
//!
//! ## Quick Tutorial
//!
//! Using this crate for making asynchronous API calls is straightforward:
//!
//! 1. initialize a [`BodhiClient`] instance:
//!
//! ```ignore
//! use bodhi::{BodhiClient, BodhiClientBuilder};
//!
//! // initialize a service for interacting with the production instance of bodhi
//! let bodhi: BodhiClient = BodhiClientBuilder::default().build().await.unwrap();
//! ```
//!
//! 2. create a request, like a [`PackageQuery`] for packages named "rust":
//!
//! ```
//! use bodhi::PackageQuery;
//!
//! // create a query for packages named "rust"
//! let package_query = PackageQuery::new().name("rust");
//! ```
//!
//! 3. pass the request to the service:
//!
//! ```ignore
//! // pass the query to bodhi, wait for the result, and don't do any error handling
//! let packages: Vec<Package> = bodhi.paginated_request(&package_query).await.unwrap();
//! ```
//!
//! ## Building Requests
//!
//! Documentation for all API calls that are available from this crate is inside the [`query`],
//! [`create`], and [`edit`] modules.
//!
//! All request instances can be constructed by following the builder pattern:
//!
//! - `::new()` associated methods take mandatory arguments (if any)
//! - `.foo()` methods take optional arguments (if any)
//!
//! For example, a [`ReleaseQuery`] has no mandatory parameters, but can be modified to exclude
//! archived releases from the query results:
//!
//! ```
//! use bodhi::ReleaseQuery;
//!
//! let query = ReleaseQuery::new().exclude_archived(true);
//! ```
//!
//! On the other hand, [`UpdateCreator`] requests are already useful with only their
//! mandatory arguments, as the optional arguments all have sane default values enforced by the
//! server:
//!
//! ```
//! use bodhi::UpdateCreator;
//! let creator =
//!     UpdateCreator::from_builds(&["rust-bodhi-1.1.1-2.fc36"], "Update for bodhi-rs 1.1.1.");
//! ```
//!
//! ## Making Requests
//!
//! All single-page requests implement the private `SingleRequest` trait, and all requests that
//! result in multi-page / paginated results implement the private `PaginatedRequest` trait. So,
//! any query that results in a single request must be passed to [`BodhiClient::request`], and any
//! query that results in multiple requests must be passed to [`BodhiClient::paginated_request`].
//! Internally, paginated requests are handled as a stream of `SingleRequest` instances, one for
//! each page of results.
//!
//! ```ignore
//! let release_query = ReleaseQuery::new().exclude_archived(true);
//! let update_creator = UpdateCreator::from_builds(&["rust-bodhi-1.1.1-2.fc36"], "Update for bodhi-rs 1.1.1.");
//!
//! let releases: Vec<Release> = bodhi.paginated_request(&release_query).await.unwrap();
//! let new_update: NewUpdate = bodhi.request(&update_creator).await.unwrap();
//! ```
//!
//! ## Changing default session parameters
//!
//! It is possible to customize some of the default behaviour of a [`BodhiClient`] by calling
//! methods on the [`BodhiClientBuilder`], using the builder pattern. The following parameters can
//! be modified:
//!
//! - request timeout duration (default: 60 seconds)
//! - retry count for failed requests (default: 3)
//! - `User-Agent` header in HTTP requests (default: `bodhi-rs v$(CARGO_PKG_VERSION)`)
//! - username and password for authenticated requests (default: unauthenticated)
//!
//! ```ignore
//! use bodhi::BodhiClientBuilder;
//!
//! let bodhi = BodhiClientBuilder::staging()
//!     .timeout(std::time::Duration::from_secs(3600))
//!     .retries(1000)
//!     .user_agent("the bodhi-rs documentation tests say hello")
//!     .authentication("janedoe", "CorrectHorseBatteryStaple")
//!     .build()
//!     .await
//!     .unwrap();
//! ```

// FIXME: the "fedora" crate is deprecated and needs to be replaced
#![allow(deprecated)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod data;
pub use data::*;

pub mod client;
pub use client::*;

pub mod error;
pub use error::*;

pub mod create;
pub use create::*;

pub mod edit;
pub use edit::*;

pub mod query;
pub use query::*;

pub(crate) mod request;

#[cfg(test)]
mod tests;

/// # release notes for all versions of this crate
#[doc = include_str!("../CHANGELOG.md")]
#[cfg(doc)]
#[allow(unused_imports)]
pub mod changelog {
    // includes for intra-doc links
    // use super::Session;
}
