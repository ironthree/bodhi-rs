use std::collections::HashMap;

use crate::data::{Build, BuildListPage};
use crate::service::BodhiService;


const DEFAULT_PAGE: i32 = 1;
// TODO: make this configurable?
const DEFAULT_ROWS: i32 = 50;


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
        BuildQuery {
            nvr: None,
            packages: None,
            releases: None,
            updates: None,
        }
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

    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Build>, String> {
        let mut builds: Vec<Build> = Vec::new();

        // load the first page of the query results
        let first = BuildPageQuery {
            page: 1,
            rows_per_page: DEFAULT_ROWS,
            nvr: self.nvr.clone(),
            packages: self.packages.clone(),
            releases: self.releases.clone(),
            updates: self.updates.clone(),
        };

        let result = first.query(bodhi)?;
        builds.extend(result.builds);

        // if there's only one page, return all we've got
        if result.pages == 1 {
            return Ok(builds);
        }

        // if there are more pages, load them all
        for page in 2..=result.pages {
            println!("Page: {} of {}", page, result.pages);

            let mut query = BuildPageQuery::new();
            query.nvr = self.nvr.clone();
            query.packages = self.packages.clone();
            query.releases = self.releases.clone();
            query.updates = self.updates.clone();
            query.page = page;

            let result = query.query(bodhi)?;
            builds.extend(result.builds);
        }

        // TODO: check if the results change while loading all pages,
        //       which would lead to missing and / or duplicate results at page boundaries

        Ok(builds)
    }
}


#[derive(Debug, Default)]
struct BuildPageQuery {
    pub nvr: Option<String>,
    pub packages: Option<Vec<String>>,
    pub releases: Option<Vec<String>>,
    pub updates: Option<Vec<String>>,
    pub page: i32,
    pub rows_per_page: i32,
}


impl BuildPageQuery {
    fn new() -> BuildPageQuery {
        BuildPageQuery {
            nvr: None,
            packages: None,
            releases: None,
            updates: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
        }
    }

    fn nvr(mut self, nvr: String) -> BuildPageQuery {
        self.nvr = Some(nvr);
        self
    }

    fn packages(mut self, packages: Vec<String>) -> BuildPageQuery {
        match &mut self.packages {
            Some(ps) => ps.extend(packages),
            None => self.packages = Some(packages),
        }

        self
    }

    fn package(mut self, package: String) -> BuildPageQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec!(package)),
        }

        self
    }

    fn releases(mut self, releases: Vec<String>) -> BuildPageQuery {
        match &mut self.releases {
            Some(rs) => rs.extend(releases),
            None => self.releases = Some(releases),
        }

        self
    }

    fn release(mut self, release: String) -> BuildPageQuery {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec!(release)),
        }

        self
    }

    fn updates(mut self, updates: Vec<String>) -> BuildPageQuery {
        match &mut self.updates {
            Some(us) => us.extend(updates),
            None => self.updates = Some(updates),
        }

        self
    }

    fn update(mut self, update: String) -> BuildPageQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec!(update)),
        }

        self
    }

    fn page(mut self, page: i32) -> BuildPageQuery {
        self.page = page;

        self
    }

    fn rows_per_page(mut self, rows_per_page: i32) -> BuildPageQuery {
        self.rows_per_page = rows_per_page;

        self
    }

    fn query(self, bodhi: &BodhiService) -> Result<BuildListPage, String> {
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

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        let mut response = bodhi.request(&path, Some(args))?;

        let builds: BuildListPage = match response.json() {
            Ok(value) => value,
            Err(error) => { return Err(format!("{:?}", error)); }
        };

        Ok(builds)
    }
}
