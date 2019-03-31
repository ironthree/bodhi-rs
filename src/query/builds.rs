use std::collections::HashMap;

use crate::data::{Build, BuildListPage};
use crate::service::BodhiService;


#[derive(Debug, Default)]
pub struct BuildNVRQuery {
    nvr: String,
}


impl BuildNVRQuery {
    pub fn new(nvr: String) -> BuildNVRQuery {
        BuildNVRQuery { nvr }
    }

    pub fn query(bodhi: &BodhiService) -> Result<Build, String> {
        unimplemented!()
    }
}


#[derive(Debug, Default)]
pub struct BuildQuery {
    nvr: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<String>>,
    updates: Option<Vec<String>>,
}


impl BuildQuery {
    pub fn new() -> BuildQuery {
        BuildQuery { nvr: None, packages: None, releases: None, updates: None }
    }

    pub fn nvr(mut self, nvr: String) -> BuildQuery {
        self.nvr = Some(nvr);
        self
    }

    pub fn package(mut self, package: String) -> BuildQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec!(package)),
        }

        self
    }

    pub fn release(mut self, release: String) -> BuildQuery {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec!(release)),
        }

        self
    }

    pub fn update(mut self, update: String) -> BuildQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec!(update)),
        }

        self
    }

    // TODO: query all pages and return the union
    // TODO: right now, only the first page (20 items) is returned
    pub fn query(self, bodhi: &BodhiService) -> Result<BuildListPage, String> {
        let path = String::from("/builds/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(nvr) = self.nvr {
            args.insert("nvr", nvr);
        }

        if let Some(packages) = self.packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(releases) = self.releases {
            args.insert("releases", releases.join(","));
        }

        if let Some(updates) = self.updates {
            args.insert("updates", updates.join(","));
        }

        /*
        // TODO: first check the number of items and then retrieve all of them
        if let Some(page) = page {
            args.insert("page", format!("{}", page));
        }

        if let Some(rpp) = rows_per_page {
            args.insert("rows_per_page", format!("{}", rpp));
        }
        */

        let mut response = bodhi.request(&path, Some(args))?;

        let builds: BuildListPage = match response.json() {
            Ok(value) => value,
            Err(error) => { return Err(format!("{:?}", error)); }
        };

        Ok(builds)
    }
}
