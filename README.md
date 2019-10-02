# bodhi REST API client

[![crates.io](https://img.shields.io/crates/v/bodhi.svg)](https://crates.io/crates/bodhi/)
[![crates.io](https://img.shields.io/crates/d/bodhi.svg)](https://crates.io/crates/bodhi/)
[![crates.io](https://img.shields.io/crates/l/bodhi.svg)](https://crates.io/crates/bodhi/)
[![docs.rs](https://docs.rs/bodhi/badge.svg)](https://docs.rs/bodhi/)

This package contains WIP rust bindings for the [bodhi] REST API.

[bodhi]: https://github.com/fedora-infra/bodhi

It uses the awesome [`reqwest`][reqwest] and [`serde`][serde] packages.

[reqwest]: https://github.com/seanmonstar/reqwest
[serde]: https://github.com/serde-rs/serde

## Current status

- All `GET` requests are implemented, and all actual API responses should successfully deserialize.
- No `POST` requests are implemented yet, because
- `OpenID` authentication is not yet implemented.

Note that the API is not finalized yet, and minor changes will still happen before the `0.1.0` release.


## TODO

- _**working**_ OpenID authentication
- implementing all POST requests for creating and editing things (WIP)

