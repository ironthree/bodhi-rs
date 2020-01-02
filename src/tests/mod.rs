#![allow(clippy::result_unwrap_used)]

use std::time::Duration;

use crate::{BodhiService, BodhiServiceBuilder};

// Longer timeout value for tests, since these queries can take a long time
const TEST_TIMEOUT: Duration = Duration::from_secs(300);

// More retries for running the tests, since they can fail quite often under load
const TEST_RETRIES: usize = 10;

fn bodhi_init() -> BodhiService {
    BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .expect("Failed to initialize bodhi service for tests.")
}

mod builds;
mod comments;
mod composes;
mod csrf;
mod dates;
mod enums;
mod overrides;
mod releases;
mod updates;
mod users;
