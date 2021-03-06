## bodhi REST API client

[![crates.io](https://img.shields.io/crates/v/bodhi.svg)](https://crates.io/crates/bodhi/)
[![crates.io](https://img.shields.io/crates/d/bodhi.svg)](https://crates.io/crates/bodhi/)
[![crates.io](https://img.shields.io/crates/l/bodhi.svg)](https://crates.io/crates/bodhi/)
[![docs.rs](https://docs.rs/bodhi/badge.svg)](https://docs.rs/bodhi/)

This crate contains rust bindings for the [bodhi] REST API as documented [here][bodhi-api].

[bodhi]: https://github.com/fedora-infra/bodhi
[bodhi-api]: https://bodhi.fedoraproject.org/docs/server_api/index.html#rest-api

It uses the awesome [`reqwest`][reqwest] and [`serde`][serde] packages, and uses [`fedora-rs`][fedora-rs] for
authenticated requests.

[reqwest]: https://github.com/seanmonstar/reqwest
[serde]: https://github.com/serde-rs/serde
[fedora-rs]: https://pagure.io/ironthree/fedora-rs


## Reporting bugs

The code makes some assumptions around the behaviour of bodhi servers. If those are wrong (either because the
implementation is wrong, or because the behaviour changed), then that is a bug. Also, there have been instances where
the schemas of responses and accepted requests has changed. If that's the case, then that's also a bug. If you
encounter (de)serialization issues, please open a bug noting the bodhi server version, and paste the failure message
(which should contain the reason for the (de)serialization failure, e.g. missing / renamed fields).


## Library design

This library tries to do error handling where reasonable, but passes server- or network-related errors through to the
caller. For example, if bodhi is under heavy load, it sometimes returns garbage responses with empty bodies. On the
other hand, there might just be a persistent network issue that makes a server request fail or time out. These are not
handled by the library, but are transparently wrapped and returned. If necessary, the request can be retried by the
caller. This library only implements a simple retry logic for network-related failures, not for client or server issues.


## Current status

- All `GET` requests are implemented, and all actual API responses should successfully deserialize.
- All `POST` requests are implemented for creating and editing items, except for creating and editing releases (since I
  have no way of testing that).


## Test coverage

Tests should pass for every commit that gets pushed to master. However, currently tests either require internet access
to check some assumptions for server behaviour, or require test data which is too big to be committed into git.

Tests are controlled with the following feature flags:

- `offline-tests`: tests able to run offline, without prerequisites (enabled by default)
- `online-tests`: tests that require internet access (for checking bodhi server behavior)
- `data-tests`: tests that require data files (data needs to be downloaded separately, but tests can run offline)


## Examples

The `examples` directory contains a few example applications to test and showcase some of the crate's functionality.


## Development

It might be helpful to enable the `debug` feature for local testing, since that enables some debugging statements at
various places in the code (mostly HTTP responses from `reqwest`).

It might also be helpful to enable building documentation for private items with `cargo doc --document-private-items`
for locally browsing all documented items.

