use std::collections::HashMap;

use reqwest::Response;
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

pub struct BodhiService {
    url: String,
}

impl BodhiService {
    pub fn new(url: String) -> BodhiService {
        BodhiService { url }
    }

    pub fn request(
        &self,
        path: &str,
        args: Option<HashMap<&str, String>>,
    ) -> Result<Response, String> {
        let client = match reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
        {
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
