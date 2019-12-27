#![allow(clippy::result_unwrap_used)]

use std::time::Duration;

// Longer timeout value for tests, since these queries can take a long time
const TEST_TIMEOUT: Duration = Duration::from_secs(300);

// More retries for running the tests, since they can fail quite often under load
const TEST_RETRIES: usize = 10;

mod builds;
mod comments;
mod composes;
mod csrf;
mod overrides;
mod releases;
mod updates;
mod users;
