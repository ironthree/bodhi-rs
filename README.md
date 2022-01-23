## bodhi REST API client

[![crates.io](https://img.shields.io/crates/v/bodhi.svg)](https://crates.io/crates/bodhi/)
[![crates.io](https://img.shields.io/crates/d/bodhi.svg)](https://crates.io/crates/bodhi/)
[![crates.io](https://img.shields.io/crates/l/bodhi.svg)](https://crates.io/crates/bodhi/)
[![docs.rs](https://docs.rs/bodhi/badge.svg)](https://docs.rs/bodhi/)

This crate contains rust bindings for the [bodhi] REST API as documented
[by the official API docs][bodhi-api].

[bodhi]: https://github.com/fedora-infra/bodhi
[bodhi-api]: https://bodhi.fedoraproject.org/docs/server_api/index.html#rest-api

The crate is based on the [fedora] crate for authenticated session support,
which uses the [`reqwest`][reqwest] crate under the hood for making network
calls, and [`serde`][serde] for (de)serializing JSON and `x-www-urlencoded`
data.

[fedora]: https://pagure.io/ironthree/fedora-rs
[reqwest]: https://github.com/seanmonstar/reqwest
[serde]: https://github.com/serde-rs/serde

## Reporting bugs

The code makes some assumptions around the behaviour of bodhi servers. If those
assumptions turn out to be wrong (either because the implementation is wrong, or
because the server behaviour has changed), then that is considered a bug. Also,
there have been instances where the schemas of responses and accepted requests
has changed. If that's the case, then that's also a bug. If you encounter
(de)serialization issues, please open a bug noting the bodhi server version, and
paste the failure message (which should contain the reason for the
(de)serialization failure, e.g. missing / renamed fields).

## Library design

This library tries to do error handling where reasonable, but passes server- or
network-related errors through to the caller. For example, when a bodhi server
is under heavy load, it sometimes returns garbage responses with empty bodies.
On the other hand, there might just be a persistent network issue that makes a
server request fail or time out. These are not handled by the library, but are
transparently wrapped and returned. If necessary, the request can be retried by
the caller. This library only implements a simple retry logic for transient
failures, not for persistent client or server issues.

However, the `BodhiClient` only takes queries by reference, so retrying a query
does not even involve copying data, so this is very cheap.

## Current status

- All `GET` requests are implemented, and all actual API responses should
  successfully deserialize.
- All `POST` requests are implemented for creating and editing items, except for
  creating and editing releases.

## Test coverage

Tests should pass for every commit that gets pushed to git. However, currently
some tests either require internet access to check some assumptions for server
behaviour, or require test data which is too big to be committed into git.

For this reason, the following feature flags determine which tests are compiled
and run:

- `offline-tests`: tests able to run offline, without prerequisites (enabled by
  default)
- `online-tests`: tests that require internet access (for checking bodhi server
  behavior)
- `data-tests`: tests that require data files (data is not part of the git
  repository or published crates and needs to be downloaded separately, but
  the tests themselves can run offline)

## Examples

The `examples` directory contains a few example applications to test and
showcase some of the crate's functionality.

