#![allow(clippy::result_unwrap_used)]

#[cfg(feature = "online-tests")]
use std::time::Duration;

#[cfg(feature = "online-tests")]
use crate::{BodhiService, BodhiServiceBuilder};

// Longer timeout value for tests, since these queries can take a long time
#[cfg(feature = "online-tests")]
const TEST_TIMEOUT: Duration = Duration::from_secs(300);

// More retries for running the tests, since they can fail quite often under load
#[cfg(feature = "online-tests")]
const TEST_RETRIES: usize = 10;

#[cfg(feature = "online-tests")]
fn bodhi_init() -> BodhiService {
    BodhiServiceBuilder::default()
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .expect("Failed to initialize bodhi service for tests.")
}

// offline tests
#[cfg(feature = "offline-tests")]
mod dates;
#[cfg(feature = "offline-tests")]
mod enums;
#[cfg(feature = "offline-tests")]
mod types;

// tests requiring internet access
#[cfg(feature = "online-tests")]
mod builds;
#[cfg(feature = "online-tests")]
mod comments;
#[cfg(feature = "online-tests")]
mod composes;
#[cfg(feature = "online-tests")]
mod csrf;
#[cfg(feature = "online-tests")]
mod overrides;
#[cfg(feature = "online-tests")]
mod releases;
#[cfg(feature = "online-tests")]
mod updates;
#[cfg(feature = "online-tests")]
mod users;
