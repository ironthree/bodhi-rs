//! This crate provides an opinionated, idiomatic, and async-aware Rust wrapper around the REST API
//! of the [bodhi] web service.
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
//! ## Quick Tutorial
//!
//! Using this crate for making asynchronous API calls is straightforward:
//!
//! 1. initialize a [`BodhiService`] instance:
//!
//! ```
//! use bodhi::{BodhiService, BodhiServiceBuilder};
//!
//! # tokio_test::block_on( async {
//! // initialize a service for interacting with the production instance of bodhi
//! let bodhi: BodhiService = BodhiServiceBuilder::default().build().await.unwrap();
//! # })
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
//! ```
//! # use bodhi::BodhiServiceBuilder;
//! # use bodhi::{Package, PackageQuery};
//! # let bodhi = tokio_test::block_on( async { BodhiServiceBuilder::default().build().await }).unwrap();
//! # let package_query = PackageQuery::new().name("rust");
//! # tokio_test::block_on( async {
//! // pass the query to bodhi, wait for the result, and don't do any error handling
//! # #[cfg(feature = "online-tests")]
//! let packages: Vec<Package> = bodhi.paginated_request(&package_query).await.unwrap();
//! # })
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
//! let creator = UpdateCreator::from_builds(&["rust-bodhi-1.1.1-2.fc36"], "Update for bodhi-rs 1.1.1.");
//! ```
//!
//! ## Making Requests
//!
//! All single-page requests implement the private `SingleRequest` trait, and all requests that
//! result in multi-page / paginated results implement the private `PaginatedRequest` trait. So,
//! any query that results in a single request must be passed to [`BodhiService::request`], and any
//! query that results in multiple requests must be passed to [`BodhiService::paginated_request`].
//! Internally, paginated requests are handled as a stream of `SingleRequest` instances, one for
//! each page of results.
//!
//! ```no_run
//! # use bodhi::BodhiServiceBuilder;
//! # use bodhi::{NewUpdate, Release, ReleaseQuery, UpdateCreator};
//! # let bodhi = tokio_test::block_on( async { BodhiServiceBuilder::default().build().await }).unwrap();
//! let release_query = ReleaseQuery::new().exclude_archived(true);
//! let update_creator = UpdateCreator::from_builds(&["rust-bodhi-1.1.1-2.fc36"], "Update for bodhi-rs 1.1.1.");
//!
//! # tokio_test::block_on( async {
//! let releases: Vec<Release> = bodhi.paginated_request(&release_query).await.unwrap();
//! let new_update: NewUpdate = bodhi.request(&update_creator).await.unwrap();
//! # })
//! ```
//!
//! ## Data type definitions
//!
//! serde all schemas structs and enums and custom release type blah blah blah FIXME
//!
//! ## Changing default session parameters
//!
//! It is possible to customize some of the default behaviour of a [`BodhiService`] by calling
//! methods on the [`BodhiServiceBuilder`], using the builder pattern. The following parameters can
//! be modified:
//!
//! - request timeout duration (default: 60 seconds)
//! - retry count for failed requests (default: 3)
//! - FIXME: User-Agent header in HTTP requests (default: "bodhi-rs v2.0.0")
//! - username and password for authenticated requests (default: unauthenticated)
//!
//! ```no_run
//! use bodhi::BodhiServiceBuilder;
//!
//! # tokio_test::block_on( async {
//! let bodhi = BodhiServiceBuilder::staging()
//!     .timeout(std::time::Duration::from_secs(3600))
//!     .retries(1000)
//!     //  .user_agent("the bodhi-rs documentation tests say hello")
//!     .authentication("janedoe", "CorrectHorseBatteryStaple")
//!     .build()
//!     .await
//!     .unwrap();
//! # })
//! ```

// ! The library is structured like this:
// !
// ! - [`BodhiService`](service/struct.BodhiService.html), which contains all information related to
// !   connecting to a remote bodhi instance
// ! - a set of `*Query` structs and implementations for querying bodhi, which wrap the REST API
// with !   a Rust-y API
// ! - a set of `Create` implementations for creating new data on bodhi
// ! - a set of `Edit` implementations for editing data on bodhi
// ! - data type and enum definitions, used for (de)serializing JSON values with [serde]
// !
// ! [serde]: https://docs.rs/serde
// !
// ! ## Data type definitions
// !
// ! The data type definitions used for deserializing the server JSON responses are contained in the
// ! `data` module. Some definitions are used only internally (for example, for deserializing
// ! paginated results), and are not publicly exported. They are located next to the `Query` they
// are ! used for.

// FIXME #![warn(missing_docs)]
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

pub(crate) mod request;

#[cfg(test)]
mod tests;
