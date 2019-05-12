use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Update};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

#[derive(Debug)]
pub struct UpdateIDQuery {
    id: String,
}

#[derive(Debug, Deserialize)]
struct UpdatePage {
    update: Update,
    can_edit: bool,
}

impl UpdateIDQuery {
    pub fn new(id: String) -> UpdateIDQuery {
        UpdateIDQuery { id }
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Update, String> {
        let path = format!("/updates/{}", self.id);

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let update: UpdatePage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(update.update)
        } else {
            let error: BodhiError = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("Unexpected error message: {:?}", error));
                }
            };

            Err(format!("{:?}", error))
        }
    }
}

#[derive(Debug, Default)]
pub struct UpdateQuery {
    active_releases: Option<bool>,
    aliases: Option<Vec<String>>,
    approved_before: Option<String>,
    approved_since: Option<String>,
    bugs: Option<Vec<String>>,
    builds: Option<Vec<String>>,
    content_type: Option<String>,
    critpath: Option<bool>,
    cves: Option<Vec<String>>,
    like: Option<String>,
    locked: Option<bool>,
    modified_before: Option<String>,
    modified_since: Option<String>,
    packages: Option<Vec<String>>,
    pushed: Option<bool>,
    pushed_before: Option<String>,
    pushed_since: Option<String>,
    releases: Option<Vec<String>>,
    request: Option<String>,
    search: Option<String>,
    severity: Option<String>,
    status: Option<String>,
    submitted_before: Option<String>,
    submitted_since: Option<String>,
    suggest: Option<String>,
    update_ids: Option<Vec<String>>,
    update_type: Option<String>,
    users: Option<Vec<String>>,
}

impl UpdateQuery {
    pub fn new() -> UpdateQuery {
        UpdateQuery {
            active_releases: None,
            aliases: None,
            approved_before: None,
            approved_since: None,
            bugs: None,
            builds: None,
            content_type: None,
            critpath: None,
            cves: None,
            like: None,
            locked: None,
            modified_before: None,
            modified_since: None,
            packages: None,
            pushed: None,
            pushed_before: None,
            pushed_since: None,
            releases: None,
            request: None,
            search: None,
            severity: None,
            status: None,
            submitted_before: None,
            submitted_since: None,
            suggest: None,
            update_ids: None,
            update_type: None,
            users: None,
        }
    }

    pub fn active_releases(mut self, active_releases: bool) -> UpdateQuery {
        self.active_releases = Some(active_releases);
        self
    }

    pub fn aliases(mut self, alias: String) -> UpdateQuery {
        match &mut self.aliases {
            Some(aliases) => aliases.push(alias),
            None => self.aliases = Some(vec![alias]),
        }

        self
    }

    pub fn approved_before(mut self, approved_before: String) -> UpdateQuery {
        self.approved_before = Some(approved_before);
        self
    }

    pub fn approved_since(mut self, approved_since: String) -> UpdateQuery {
        self.approved_since = Some(approved_since);
        self
    }

    pub fn bugs(mut self, bug: String) -> UpdateQuery {
        match &mut self.bugs {
            Some(bugs) => bugs.push(bug),
            None => self.bugs = Some(vec![bug]),
        }

        self
    }

    pub fn builds(mut self, build: String) -> UpdateQuery {
        match &mut self.builds {
            Some(builds) => builds.push(build),
            None => self.builds = Some(vec![build]),
        }

        self
    }

    pub fn content_type(mut self, content_type: String) -> UpdateQuery {
        self.content_type = Some(content_type);
        self
    }

    pub fn critpath(mut self, critpath: bool) -> UpdateQuery {
        self.critpath = Some(critpath);
        self
    }

    pub fn cves(mut self, cve: String) -> UpdateQuery {
        match &mut self.cves {
            Some(cves) => cves.push(cve),
            None => self.cves = Some(vec![cve]),
        }

        self
    }

    pub fn like(mut self, like: String) -> UpdateQuery {
        self.like = Some(like);
        self
    }

    pub fn locked(mut self, locked: bool) -> UpdateQuery {
        self.locked = Some(locked);
        self
    }

    pub fn modified_before(mut self, modified_before: String) -> UpdateQuery {
        self.modified_before = Some(modified_before);
        self
    }

    pub fn modified_since(mut self, modified_since: String) -> UpdateQuery {
        self.modified_since = Some(modified_since);
        self
    }

    pub fn packages(mut self, package: String) -> UpdateQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    pub fn pushed(mut self, pushed: bool) -> UpdateQuery {
        self.pushed = Some(pushed);
        self
    }

    pub fn pushed_before(mut self, pushed_before: String) -> UpdateQuery {
        self.pushed_before = Some(pushed_before);
        self
    }

    pub fn pushed_since(mut self, pushed_since: String) -> UpdateQuery {
        self.pushed_since = Some(pushed_since);
        self
    }

    pub fn releases(mut self, release: String) -> UpdateQuery {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec![release]),
        }

        self
    }

    pub fn request(mut self, request: String) -> UpdateQuery {
        self.request = Some(request);
        self
    }

    pub fn search(mut self, search: String) -> UpdateQuery {
        self.search = Some(search);
        self
    }

    pub fn severity(mut self, severity: String) -> UpdateQuery {
        self.severity = Some(severity);
        self
    }

    pub fn status(mut self, status: String) -> UpdateQuery {
        self.status = Some(status);
        self
    }

    pub fn submitted_before(mut self, submitted_before: String) -> UpdateQuery {
        self.submitted_before = Some(submitted_before);
        self
    }

    pub fn submitted_since(mut self, submitted_since: String) -> UpdateQuery {
        self.submitted_since = Some(submitted_since);
        self
    }

    pub fn suggest(mut self, suggest: String) -> UpdateQuery {
        self.suggest = Some(suggest);
        self
    }

    pub fn update_ids(mut self, update_id: String) -> UpdateQuery {
        match &mut self.update_ids {
            Some(update_ids) => update_ids.push(update_id),
            None => self.update_ids = Some(vec![update_id]),
        }

        self
    }

    pub fn update_type(mut self, update_type: String) -> UpdateQuery {
        self.update_type = Some(update_type);
        self
    }

    pub fn users(mut self, user: String) -> UpdateQuery {
        match &mut self.users {
            Some(users) => users.push(user),
            None => self.users = Some(vec![user]),
        }

        self
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Update>, String> {
        let mut updates: Vec<Update> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = UpdatePageQuery::new();
            query.page = page;

            query.active_releases = self.active_releases;
            query.aliases = self.aliases.clone();
            query.approved_before = self.approved_before.clone();
            query.approved_since = self.approved_since.clone();
            query.bugs = self.bugs.clone();
            query.builds = self.builds.clone();
            query.content_type = self.content_type.clone();
            query.critpath = self.critpath;
            query.cves = self.cves.clone();
            query.like = self.like.clone();
            query.locked = self.locked;
            query.modified_before = self.modified_before.clone();
            query.modified_since = self.modified_since.clone();
            query.packages = self.packages.clone();
            query.pushed = self.pushed;
            query.pushed_before = self.pushed_before.clone();
            query.pushed_since = self.pushed_since.clone();
            query.releases = self.releases.clone();
            query.request = self.request.clone();
            query.search = self.search.clone();
            query.severity = self.severity.clone();
            query.status = self.status.clone();
            query.submitted_before = self.submitted_before.clone();
            query.submitted_since = self.submitted_since.clone();
            query.suggest = self.suggest.clone();
            query.update_ids = self.update_ids.clone();
            query.update_type = self.update_type.clone();
            query.users = self.users.clone();

            let result = query.query(bodhi)?;
            updates.extend(result.updates);

            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(updates)
    }
}

#[derive(Debug, Deserialize)]
struct UpdateListPage {
    updates: Vec<Update>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}

#[derive(Debug)]
struct UpdatePageQuery {
    active_releases: Option<bool>,
    aliases: Option<Vec<String>>,
    approved_before: Option<String>,
    approved_since: Option<String>,
    bugs: Option<Vec<String>>,
    builds: Option<Vec<String>>,
    content_type: Option<String>,
    critpath: Option<bool>,
    cves: Option<Vec<String>>,
    like: Option<String>,
    locked: Option<bool>,
    modified_before: Option<String>,
    modified_since: Option<String>,
    packages: Option<Vec<String>>,
    pushed: Option<bool>,
    pushed_before: Option<String>,
    pushed_since: Option<String>,
    releases: Option<Vec<String>>,
    request: Option<String>,
    search: Option<String>,
    severity: Option<String>,
    status: Option<String>,
    submitted_before: Option<String>,
    submitted_since: Option<String>,
    suggest: Option<String>,
    update_ids: Option<Vec<String>>,
    update_type: Option<String>,
    users: Option<Vec<String>>,

    page: i32,
    rows_per_page: i32,
}

impl UpdatePageQuery {
    fn new() -> UpdatePageQuery {
        UpdatePageQuery {
            active_releases: None,
            aliases: None,
            approved_before: None,
            approved_since: None,
            bugs: None,
            builds: None,
            content_type: None,
            critpath: None,
            cves: None,
            like: None,
            locked: None,
            modified_before: None,
            modified_since: None,
            packages: None,
            pushed: None,
            pushed_before: None,
            pushed_since: None,
            releases: None,
            request: None,
            search: None,
            severity: None,
            status: None,
            submitted_before: None,
            submitted_since: None,
            suggest: None,
            update_ids: None,
            update_type: None,
            users: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
        }
    }

    fn query(self, bodhi: &BodhiService) -> Result<UpdateListPage, String> {
        let path = String::from("/updates/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(active_releases) = self.active_releases {
            args.insert("active_releases", active_releases.to_string());
        };

        if let Some(aliases) = self.aliases {
            args.insert("alias", aliases.join(","));
        };

        if let Some(approved_before) = self.approved_before {
            args.insert("approved_before", approved_before);
        };

        if let Some(approved_since) = self.approved_since {
            args.insert("approved_since", approved_since);
        };

        if let Some(bugs) = self.bugs {
            args.insert("bugs", bugs.join(","));
        };

        if let Some(builds) = self.builds {
            args.insert("builds", builds.join(","));
        };

        if let Some(content_type) = self.content_type {
            args.insert("content_type", content_type);
        };

        if let Some(critpath) = self.critpath {
            args.insert("critpath", critpath.to_string());
        };

        if let Some(cves) = self.cves {
            args.insert("cves", cves.join(","));
        };

        if let Some(like) = self.like {
            args.insert("like", like);
        };

        if let Some(locked) = self.locked {
            args.insert("locked", locked.to_string());
        };

        if let Some(modified_before) = self.modified_before {
            args.insert("modified_before", modified_before);
        };

        if let Some(modified_since) = self.modified_since {
            args.insert("modified_since", modified_since);
        };

        if let Some(packages) = self.packages {
            args.insert("packages", packages.join(","));
        };

        if let Some(pushed) = self.pushed {
            args.insert("pushed", pushed.to_string());
        };

        if let Some(pushed_before) = self.pushed_before {
            args.insert("pushed_before", pushed_before);
        };

        if let Some(pushed_since) = self.pushed_since {
            args.insert("pushed_since", pushed_since);
        };

        if let Some(releases) = self.releases {
            args.insert("releases", releases.join(","));
        };

        if let Some(request) = self.request {
            args.insert("request", request);
        };

        if let Some(search) = self.search {
            args.insert("search", search);
        };

        if let Some(severity) = self.severity {
            args.insert("severity", severity);
        };

        if let Some(status) = self.status {
            args.insert("status", status);
        };

        if let Some(submitted_before) = self.submitted_before {
            args.insert("submitted_before", submitted_before);
        };

        if let Some(submitted_since) = self.submitted_since {
            args.insert("submitted_since", submitted_since);
        };

        if let Some(suggest) = self.suggest {
            args.insert("suggest", suggest);
        };

        if let Some(update_ids) = self.update_ids {
            args.insert("updateid", update_ids.join(","));
        };

        if let Some(update_type) = self.update_type {
            args.insert("type", update_type);
        };

        if let Some(users) = self.users {
            args.insert("user", users.join(","));
        };

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        let mut response = bodhi.request(&path, Some(args))?;
        let status = response.status();

        if status.is_success() {
            let updates: UpdateListPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(updates)
        } else {
            let error: BodhiError = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("Unexpected error message: {:?}", error));
                }
            };

            Err(format!("{:?}", error))
        }
    }
}
