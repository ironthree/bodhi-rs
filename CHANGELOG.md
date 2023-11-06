### Release 2.1.1

- Derive `Default` implementations for enums where possible.
- Revert back to using rpassword v5 in example code for now.

### Release 2.1.0

This release contains a few small improvements and updates to match
changes in bodhi server code.

Added:

- added `eol` field to the `Release` struct
- added `critpath_gtoups` field to the `Update` struct

Changed:

- dropped the `oldtime` feature of `chrono` to remove the dependency on the
  obsolete version 0.1 of the `time` crate
- adapted tests for new and out-of-date Fedora releases
- mark structs with the `#[non_exhaustive]` attribute instead of using
  private fields
- ported from `lazy_static` to `once_cell`

Fixed:

- fixed the `deprecated(since)` annotation for the `composes` field of the
  `Release` struct

### Release 2.0.1 "Adaptations" (May 18, 2022)

This release contains a few small improvements and fixes for upcoming bodhi
server behaviour changes:

The OpenID endpoint for logging into bodhi servers will be deprecated with bodhi
6.0 servers, but will remain available under a different URL for now. The
primary method of authentication will be OpenID Connect, which is not going to
be supported in the `fedora` crate anytime soon, as it does not look like it
will support headless authentication, but will require the user to sign on via a
web browser window to retrieve an authentication token.  With version 2.0.1 of
this crate, the authentication method was changed to explicitly use the
deprecated OpenID endpoint, until OpenID connect support can be implemented in a
way that works for this crate and its users.

Additionally, bodhi servers will no longer include the `composes` field when
serializing `Release` objects. As a result, the struct field has been marked as
`deprecated`, but otherwise it remains the same for backwards compatibility.
However, because JSON server responses will no longer include this field, it
will always contain a value of `None`, and should no longer be used.

The changes since 2.0.0 also include additions to the test dataset and
test suite that cover the latest Fedora releases:

- remove Fedora 33 and EPEL-6 from the lists of active releases
- add missing tests and test data for Fedora 36 and 37 releases

### Release 2.0.0 "Finally" (February 01, 2022)

This release contains no code changes compared to the previous beta. The only
changes are some updated crate dependencies (to match the versions that are
available from Fedora repositories at the time of publishing), and simplified
doctests (making it possible to drop the dev-dependency on `tokio-test`, which
was only used in some doctests that were not even run, but only compiled).

For a complete list of changes since `v1.1.1`, read the release notes for the
last two beta releases.

### Release 2.0.0-beta.2 "Spring Cleaning" (January 28, 2022)

This beta release only includes some code cleanups and small improvements for
error messages and documentation.

### Release 2.0.0-beta.1 "Modern Times" (January 23, 2022)

This version is an almost-complete rewrite of the entire crate, with
numerous changes and improvements.

Most notably, all network calls that use the underlying `fedora` crate are now
non-blocking / `async`.

The API for making requests has been completely rewritten with simplified
traits, and it is now explicit when which requests will be processed as
paginated queries based on the trait they implement.

The rewritten API also makes it very easy to expose requests for specific pages
of paginated results, which was not the case in older versions of this crate.

The `FedoraRelease` type has been converted from an enumerated value (which
made it necessary to release new versions of this crate for new Fedora or EPEL
releases) to a newtype wrapper around string types with a validated format. This
is intended to be a more future-proof solution, as only the format of release
identifier strings is now validated, but the strings themselves are not compared
to a list of hard-coded known release identifiers.

Overview of changes since version 1.1.1:

- ported to use asynchronous network calls exposed by `fedora` v2.0.0+
- ported all examples and tests to use `#[tokio::test]`, though all library code
  of this crate is runtime-agnostic
- use re-exported `reqwest` and `url` from `fedora`, since those crates are
  part of the public API and version needs to match the one exposed by the 
  `fedora` crate
- renamed `BodhiService` to `BodhiClient`, as that matches what it does
- removed all specialized traits for `Query` / `PaginatedQuery`, `Create`, and
  `Edit` in favor of using only `Request` and `PaginatedRequest`
- simplified error types (removed `ServiceError`, its variants have been merged
  into the `QueryError` type), and added a specific error for invalid input data
- reimplemented `FedoraRelease` enum as a newtype wrapper around string types,
  making the crate more future-proof, as this does not require code changes for
  supporting new Fedora or EPEL releases every few months
- make it impossible to construct most "output-only" types outside this crate
- expose APIs for running single-page queries for specific result pages

Development-specific changes:

- python scripts for generating integration tests have been rewritten to use
  Jinja2 templates, making the code moch more easy to maintain or adapt
- added property-based tests for validating the regular expressions used to
  parse and validate `FedoraRelease` values from strings

Additionally, every line of documentation has been rewritten from scratch to
make sure it matches the latest state of the crate. The `NEWS.md` changelog is
now also imported into the crate's documentation.

### Release 1.1.1 "Corporate Badger" (November 30, 2021)

Changes:

- add EPEL 9 (`EPEL-9` and `EPEL-9N`) variants to `FedoraRelease`
- add workarounds for slightly broken generated deserialization tests

### Release 1.1.0 "Cookie Monster" (September 23, 2021)

Changes:

- require `fedora = ^1.1` for better OpenID session / cookie cache support

### Release 1.0.5 "Moar Numbers" (August 11, 2021)

Improvements:

- add all F35 and F36 variants to `FedoraRelease`

### Release 1.0.4 "Number Next" (May 13, 2021)

Improvements:

- add `EPEL8N` as valid `FedoraRelease` variant

### Release 1.0.3 "Modular Flatpak Containers" (March 6, 2021)

Improvements:

- add `F35` as valid `FedoraRelease` variant
- add `F35C` as valid `FedoraRelease` variant
- add `F34F` as valid `FedoraRelease` variant
- add `F34M` as valid `FedoraRelease` variant

### Release 1.0.2 "Up This Grade" (January 5, 2021)

Internal changes:

- updated `reqwest` from `0.10` to `0.11`
- updated `fedora` from `0.2.1` to `1.0`
- port from `failure` to `thiserror`

### Release 1.0.1 "Tranquility" (November 30, 2020)

This is a routine update with no user-facing changes.

The attached test data for offline tests contains a data dump of the Fedora
bodhi instance from Nov. 30, 2020, just after the fedora 31 EOL date.

### Release 1.0.0 "Stability" (November 16, 2020)

This is the first "stable" release of bodhi-rs.

Compared to 0.6.4, it brings only some minor API changes, primarily to no longer
hide some Vec allocations, which allows writing more efficient code in some
situations: Query arguments that allow multiple arguments now take a Vec of
values directly instead of being callable multiple times. Of course, all
examples and tests were adapted to reflect this change.

### Release 0.6.4 "Exclusivity" (October 29, 2020)

This release only contains non-code changes:

- exclude both the `.github` folder and `rustfmt.toml` from published crates

### Release 0.6.3 "Thirty-Four Containers" (Oct. 27, 2020)

Improvements:

- `F34C` as valid `FedoraRelease` variant
- minor code cleanups thanks to clippy

### Release 0.6.2 "Groundhog Day" (Aug. 27, 2020)

Improvements:

- add `F33F` as valid `FedoraRelease` variant
- add `F33M` as valid `FedoraRelease` variant

### Release 0.6.1 "Murmeltier" (Aug. 12, 2020)

Improvements:

- add `F34` as valid `FedoraRelease` variant

### Release 0.6.0 "Upstream" (June 22, 2020)

Bug fixes:

- adapt to bodhi API response schema changes (causes a minor API change)

### Release 0.5.10 "Elastin" (May 20, 2020)

Improvements:

- add `ELN` as valid `FedoraRelease` variant

### Release 0.5.9 "Macros" (Mar. 25, 2020)

New features:

- add new magic `__archived__`, `__pending__`, and `__current__` releases

They can be used as shortcuts when querying updates for currently archived,
pending, or current releases.

### Release 0.5.8 "Bring Me 'nother!" (Mar. 20, 2020)

Improvements:

- add `F32F` as valid `FedoraRelease` variant

### Release 0.5.7 "And 'Nother One!" (Mar. 05, 2020)

Improvements:

- add `F33C` as valid `FedoraRelease` variant

### Release 0.5.6 "Another Release Date" (Feb. 29, 2020)

Improvements:

- add `F32M` as valid `FedoraRelease` variant

### Release 0.5.5 "Release Date" (Feb. 16, 2020)

Improvements:

- add `F33` as valid `FedoraRelease` variant
- add `F33` test targets

### Release 0.5.4 "Link" (Jan. 26, 2020)

New features:

- add `url()` method on `Bug` (link to Red Hat Bugzilla ticket)
- add `url()` method on `TestCase` (link to <https://fedoraproject.org> wiki)

### Release 0.5.3 "Dependence Day" (Jan. 25, 2020)

Bump `retry` and `serde_url_params` dependencies.

### Release 0.5.2 "Pauli Exclusion Principle" (Jan. 24, 2020)

Fix `exclude` pattern in `Cargo.toml` so only the `/tests` directory in the
project root is excluded, and not the `src/tests` module. Now tests can be
run from the crate downloaded from <https://crates.io>.

### Release 0.5.1 "Constructive Feedback" (Jan. 23, 2020)

Finally: Fixed providing bug and testcase feedback with comments.

### Release 0.5.0 "Argument Alignment" (Jan. 19, 2020)

Bugfixes:

- rename `title` and `display_name` attributes of `Updates`, following upstream
  changes (`title` is now a computed attribute)
- no longer lose custom `display_name` attribute of updates when editing them
- rename methods for querying and editing `title` to `display_name` accordingly

These changes affected the public API, so the version was bumped to indicate an
incompatible change, as well. 

### Release 0.4.0 "Parse me" (Jan. 17, 2020)

New features:

- implement `TryFrom<&str>` and `FromStr` for every enumerated type

This allows using `str::parse()` for all enum values (for example, for
`structopt` arguments).

Refactored test suite into multiple features:

- `offline-tests`: enabled by default
- `online-tests`: require internet access, disabled by default
- `data-tests`: deserialization tests, require local data, disabled by default

### Release 0.3.2 "Silence" (Jan. 17, 2020)

Bugfixes:

- remove a stray `println!()` call left over from debugging

### Release 0.3.1 "Revert! Revert!" (Jan. 16, 2020)

Revert breaking API changes from the 0.3.0 release.

### Release 0.3.0 "Hurr Durr I'ma Sheep" (Jan. 15, 2020)

**DO NOT USE THIS RELEASE**

### Release 0.2.3 "From this Date" (Jan. 15, 2020)

New features:

- implement `TryFrom<DateTime<Utc>>` for `BodhiDate` (it's only a wrapper ...)

### Release 0.2.2 "In and Out" (Jan. 14, 2020)

New features:

- derive `Serialize` for all data types, as well
- new `Serialize` implementation for `BodhiDate` 

### Release 0.2.1 "Start at the Beginning" (Jan. 12, 2020)

Small improvements:

- if present, invoke progress callback function first with `0/?` as well

### Release 0.2.0 "Progress Report" (Jan. 11, 2020)

Small improvements:

- add possibility to supply a progress callback to long-running queries
- more attempts to fix submitting bug and testcase feedback with comments

### Release 0.1.7 "Null Pointer Dereference" (Jan. 09, 2020)

Small improvements:

- more fixes creating test case feedback

### Release 0.1.6 "Speak, friend, and enter" (Jan. 09, 2020)

Small improvements:

- add example for waiving update test results
- add example for requesting update state change
- start fixing code for providing `Bug` and `TestCase` feedback with comments
- implement `Eq` and `Ord` for `BodhiDate`, so things can be sorted by dates

### Release 0.1.5 "Keep this secret" (Jan. 04, 2020)

Small improvements:

- clean up examples and properly read password from commandline with `rpassword`
- some more `impl Display` cleanups
- add convenience methods for directly creating update status requests and
  result waive requests from an existing `Update`

### Release 0.1.4 "Quantum Leap" (Jan. 04, 2020)

Small improvements:

- removed `Checkpoints` struct, which was useless and not working
- cleaned up `Display` implementations for structs

### Release 0.1.3 "Go home, you're drunk" (Jan. 04, 2020)

Bugfixes:

- manually implement `Display` for enums instead of relying on
  `serde_json::to_string()`, which doesn't work as expected 

### Release 0.1.2 "Convini" (Jan. 03, 2020)

Include some new convenience methods:

- commenting on an existing `Update`
- creating a buildroot override from an existing `Build`
- editing a buildroot override from an existing `Override`
- editing an update from an existing `Update`

Other small improvements:

- fixed `impl Display` for `FedoraRelease`
- make some internally used items private again
- method to construct URL of a BugZilla bug for `Bug` instances

### Release 0.1.1 "Try Again" (Jan. 02, 2020)

Small improvements:

- mark `FedoraRelease` `enum` as `#[non_exhaustive]`
- implement `TryFrom<&str>` for `FedoraRelease`

### Release 0.1.0 "Fully Functional" (Jan. 01, 2020)

Multiple changes and improvements:

- refactored public API to be more approachable
- take references instead of owned types, where possible
- implementations for querying, creating, and editing done (and working)
- added a lot of tests to check both data type definitions and query logic

The test data is not committed into git, since it's 2GB+ large.

### Release 0.0.8 (Dec. 12, 2019)

Incremental improvements:

- add missing `EPEL-8M` release to list and tests
- adapt to `fedora-rs` API changes with release 0.0.7

### Release 0.0.7 (Nov. 30, 2019)

Incremental improvements:

- implement simple conversions from strings to enums and vice versa

### Release 0.0.6 (Nov. 30, 2019)

Incremental improvements:

- introduce pretty error handling that does more than string concatenation
- simplify code for queries
- start implementations for creating comments and overrides
- adapt to bodhi 4.1.0 changes and new fedora release targets

### Release 0.0.5 (May 29, 2019)

Incremental improvements:

- fix copypasta issue in `UpdateType` enum definition
- simplify code for disambiguating "not found" from "error" situations
- retry queries if the server returned an error
- more complete test coverage
- adapt struct definitions to changed JSON response schemas with bodhi 4.0.0

### Release 0.0.4 (May 24, 2019)

Incremental improvements:

- abstract over inconsistent Update ID type (numerical ID vs. string alias)
- add small example program for showing off API usage
- refactor tests

### Release 0.0.3 (May 23, 2019)

Incremental improvements:

- return `None` if querying for exactly one item by ID / name / NVR and no item
  with that identifier exists

### Release 0.0.2 (May 18, 2019)

Small maintenance release with incremental improvements:

- implement clippy suggestions
- move tests from `bodhi` binary to `tests` module
- define `enum`s for common enumerated values
- deserialize JSON strings to these new `enum`s
- rename struct fields that are keywords in rust (`type`, `override`, etc.)

### Release 0.0.1 (May 13, 2019)

Initial Release:

- all `GET` requests for queries are implemented and working
- all public items are documented

