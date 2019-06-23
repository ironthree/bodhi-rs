//! This module contains all the REST API query wrappers that attempt to map
//! the REST-y API to an idiomatic Rust API, using builder patterns to
//! construct complex queries.
//!
//! For example, the bodhi REST API accepts `Sequence` arguments for certain
//! keywords, which have to be encoded as comma-separated lists of Strings.
//! The Rust API abstracts this as with methods on the query that accept normal
//! Strings and then construct the comma-separated lists on demand.

pub mod builds;
pub use builds::*;

pub mod comments;
pub use comments::*;

pub mod csrf;
pub use csrf::*;

pub mod overrides;
pub use overrides::*;

pub mod packages;
pub use packages::*;

pub mod releases;
pub use releases::*;

pub mod updates;
pub use updates::*;

pub mod users;
pub use users::*;

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use crate::error::{BodhiError, QueryError};
use crate::service::BodhiService;

pub(crate) fn retry_query(
    bodhi: &BodhiService,
    path: &str,
    args: HashMap<&str, String>,
) -> Result<String, QueryError> {
    // retry once and keep track of errors
    // bodhi returns non-JSON responses in rare circumstances
    let mut retries = 2;
    let mut errors: Vec<QueryError> = Vec::new();

    loop {
        if retries == 0 {
            break;
        }

        let mut response = bodhi.get(&path, Some(args.clone()))?;
        let status = response.status();

        if status.is_success() {
            let value = match response.text() {
                Ok(value) => value,
                Err(error) => {
                    // request successful but other error occurred
                    retries -= 1;
                    errors.push(QueryError::RequestError { error });
                    sleep(Duration::from_secs(1));
                    continue;
                }
            };

            return Ok(value);
        } else {
            let message = response.text()?;

            let error: BodhiError = match serde_json::from_str(&message) {
                Ok(value) => value,
                Err(error) => {
                    retries -= 1;
                    errors.push(error.into());
                    sleep(Duration::from_secs(1));
                    continue;
                }
            };

            // bodhi returned an error message
            retries -= 1;
            errors.push(QueryError::BodhiError { error });
            sleep(Duration::from_secs(1));
            continue;
        }
    }

    // return the last error
    Err(errors.into_iter().last().unwrap())
}
