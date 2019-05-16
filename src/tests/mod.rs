use std::time::Duration;

/// URL of the fedora bodhi instance, for running tests
const SERVER_URL: &str = "https://bodhi.fedoraproject.org";

/// Longer timeout value for tests, since these queries can take a long time
const TEST_TIMEOUT: Duration = Duration::from_secs(120);

/// More retries for running the tests, since they fail quite often
const TEST_RETRIES: u32 = 5;

mod builds;
mod overrides;
mod packages;
mod releases;
mod stacks;
mod updates;
mod users;
