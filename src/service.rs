use std::collections::HashMap;

use reqwest::Response;


pub struct BodhiService {
    url: String,
}


impl BodhiService {
    pub fn new(url: String) -> BodhiService {
        BodhiService { url }
    }

    pub fn request(&self, path: &str, args: Option<HashMap<&str, String>>) -> Result<Response, String> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}", &self.url, path);

        let query: Vec<(&str, String)> = match args {
            Some(mut map) => { map.drain().collect() }
            None => { Vec::new() }
        };

        let mut response = match client.get(&url).query(&query).send() {
            Ok(response) => response,
            Err(error) => { return Err(format!("{:#?}", error)); }
        };

        if !response.status().is_success() {
            return Err(format!("{:#?}", response.text()));
        };

        Ok(response)
    }

    // TODO: method for POST requests
}
