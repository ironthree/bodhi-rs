## bodhi REST API client

[![crates.io](https://img.shields.io/crates/v/bodhi.svg)](https://crates.io/crates/bodhi/)
[![crates.io](https://img.shields.io/crates/d/bodhi.svg)](https://crates.io/crates/bodhi/)
[![crates.io](https://img.shields.io/crates/l/bodhi.svg)](https://crates.io/crates/bodhi/)
[![docs.rs](https://docs.rs/bodhi/badge.svg)](https://docs.rs/bodhi/)

This package contains WIP rust bindings for the [bodhi] REST API as documented [here][bodhi-api].

[bodhi]: https://github.com/fedora-infra/bodhi
[bodhi-api]: https://bodhi.fedoraproject.org/docs/server_api/index.html#rest-api

It uses the awesome [`reqwest`][reqwest] and [`serde`][serde] packages, and is based on [`fedora-rs`][fedora-rs].

[reqwest]: https://github.com/seanmonstar/reqwest
[serde]: https://github.com/serde-rs/serde
[fedora-rs]: https://github.com/ironthree/fedora-rs


## Library design

This library tries to do error handling where reasonable, but passes server- or network-related errors through to the
caller. For example, if bodhi is under heavy load, it sometimes returns garbage responses with empty bodies. On the
other hand, there might just be a transient network issue that makes a server request fail or time out. These are not
handled by the library, but are transparently wrapped and returned. If necessary, the request can be retried by the
caller, but this library does not implement any retry logic itself (yet).


## Current status

- All `GET` requests are implemented, and all actual API responses should successfully deserialize.
- `POST` requests are work-in-progress.
- refactoring around common traits with common logic is constantly happening

Note that the API is not finalized yet, and minor changes will still happen before the `1.0.0` release.


## TODO

- POST requests for creating things
    - overrides (WIP)
    - releases (REALLY?)
    - updates (NSY)
    - updates/request (NSY)
    - updates/waive-test-results (NSY)
    - updates/get-test-results (NSY)

- POST requests for editing things
    - overrides (NSY)
    - releases (REALLY?)
    - updates (NSY)

- use `chrono` for date/time handling and conversions from/to strings instead of supplying date
  arguments as strings (with `chrono/serde` feature)


## Test coverage

Tests should pass for every commit that gets pushed to master. By default, only "fast" tests are run by `cargo test`,
where "fast" means that they should finish within a minute or so.

Before pushing, the "slow" tests are also run once with `cargo test --features slow_tests`. These make sure that this
library can still successfully deserialize all JSON server responses. These tests take more than an hour to complete.

There are also some "ignored" tests, which cannot ever feasibly be run regularly, since they basically read the complete
database of the bodhi instance, which takes *ages*. These tests are also set up to run against the staging instance
of bodhi, so the production instance isn't DOSed.


## Examples

The `examples` directory contains a few example applications to test and showcase some library functionality. Currently,
this includes a simple program to file a comment or create a buildroot override in the staging instance of bodhi.


## Development

It might be helpful to enable the `debug` feature for local testing, since that enables some debugging statements at
various places in the code (mostly HTTP responses from `reqwest`).

It might also be helpful to enable building documentation for private items with `cargo doc --document-private-items`
for locally browsing all documented items.
