use std::time::Duration;

// Longer timeout value for tests, since these queries can take a long time
const TEST_TIMEOUT: Duration = Duration::from_secs(120);

// More retries for running the tests, since they fail quite often
const TEST_RETRIES: u32 = 5;

mod builds;
mod comments;
mod csrf;
mod overrides;
mod packages;
mod releases;
mod updates;
mod users;
