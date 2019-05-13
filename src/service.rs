use std::collections::HashMap;

use reqwest::Response;
use std::time::Duration;

/// Always start with page 1 for multi-page queries.
/// Everything else would be stupid.
pub const DEFAULT_PAGE: i32 = 1;

/// This constant defines how many items are queried every time for multi-page queries.
/// The maximum is 100, the default is 20, and 50 seems a good compromise for speed.
pub const DEFAULT_ROWS: i32 = 50;

/// Specify a longer timeout duration (60 s) for bodhi requests.
/// The `reqwest` default value of 30 s is a bit too short for long-running queries.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// This struct represents a specific bodhi service, typically running remotely,
/// although a local URL could be specified, as well.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from("https://bodhi.fedoraproject.org"));
/// ```
#[derive(Debug)]
pub struct BodhiService {
    url: String,
    timeout: Duration,
}

impl BodhiService {
    /// This method constructs a new `BodhiService` instance.
    pub fn new(url: String) -> BodhiService {
        BodhiService { url, timeout: REQUEST_TIMEOUT }
    }

    /// This method allows to override the default timeout value (60 seconds).
    pub fn timeout(mut self, timeout: Duration) -> BodhiService {
        self.timeout = timeout;
        self
    }

    /// This method constructs and executes a request at the specified bodhi instance.
    pub fn request(
        &self,
        path: &str,
        args: Option<HashMap<&str, String>>,
    ) -> Result<Response, String> {
        let client = match reqwest::Client::builder().timeout(self.timeout).build() {
            Ok(client) => client,
            Err(error) => {
                return Err(format!("TLS backend could not be initialized: {:?}", error));
            }
        };

        let url = format!("{}/{}", &self.url, path);

        let query: Vec<(&str, String)> = match args {
            Some(mut map) => map.drain().collect(),
            None => Vec::new(),
        };

        let response = match client.get(&url).query(&query).send() {
            Ok(response) => response,
            Err(error) => {
                return Err(format!("{:#?}", error));
            }
        };

        Ok(response)
    }

    // TODO: OpenID client authentication
    // TODO: method for POST requests
}
