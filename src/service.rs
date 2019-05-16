use std::collections::HashMap;

use reqwest::Response;
use std::thread::sleep;
use std::time::Duration;

/// Always start with page 1 for multi-page queries.
/// Everything else would be stupid.
pub const DEFAULT_PAGE: u32 = 1;

/// This constant defines how many items are queried every time for multi-page queries.
/// The maximum is 100, the default is 20, and 50 seems a good compromise for speed.
pub const DEFAULT_ROWS: u32 = 50;

/// Specify a longer timeout duration (60 s) for bodhi requests.
/// The `reqwest` default value of 30 seconds is a bit too short for long-running queries.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Specify a number of retries in case of connection failures.
const REQUEST_RETRIES: u32 = 3;

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
    retries: u32,
}

impl BodhiService {
    /// This method constructs a new `BodhiService` instance.
    pub fn new(url: String) -> BodhiService {
        BodhiService {
            url,
            timeout: REQUEST_TIMEOUT,
            retries: REQUEST_RETRIES,
        }
    }

    /// This method allows to override the default timeout value (60 seconds).
    pub fn timeout(mut self, timeout: Duration) -> BodhiService {
        self.timeout = timeout;
        self
    }

    /// This method allows to override the default number of retries (3).
    pub fn retries(mut self, retries: u32) -> BodhiService {
        self.retries = retries;
        self
    }

    /// This method constructs and executes a request at the specified bodhi instance.
    pub fn request(
        &self,
        path: &str,
        args: Option<HashMap<&str, String>>,
    ) -> Result<Response, String> {
        // construct custom client (with longer timeout)
        let client = match reqwest::Client::builder().timeout(self.timeout).build() {
            Ok(client) => client,
            Err(error) => {
                return Err(format!("TLS backend could not be initialized: {:?}", error));
            }
        };

        // construct url and query
        let url = format!("{}/{}", &self.url, path);

        let query: Vec<(&str, String)> = match args {
            Some(mut map) => map.drain().collect(),
            None => Vec::new(),
        };

        // retry in case of connection failures
        let mut retries = REQUEST_RETRIES;

        while retries > 0 {
            if let Ok(response) = client
                .get(&url)
                .header(reqwest::header::USER_AGENT, "bodhi-rs")
                .query(&query)
                .send()
            {
                match response.content_length() {
                    None => {
                        // response is empty
                        sleep(Duration::from_secs(1));
                        retries -= 1;
                    }
                    Some(_len) => {
                        // return the first valid response
                        return Ok(response);
                    }
                }
            } else {
                // take a breath, and keep on trying (or not)
                sleep(Duration::from_secs(1));
                retries -= 1;
            }
        }

        // fail if the connection keeps failing
        Err(format!(
            "Failed to query bodhi instance ({} retries).",
            REQUEST_RETRIES
        ))
    }

    // TODO: OpenID client authentication
    // TODO: method for POST requests
}
