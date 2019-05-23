//! query for updates (or *one* update by ID, title, or alias)
//!
//! The contents of this module can be used to query a bodhi instance about
//! existing updates.
//!
//! The `UpdateIDQuery` returns exactly one Update, if and only if a Update
//! with this ID, alias, or title exists - otherwise, it will return an error.
//!
//! The `UpdateQuery` can be used to execute more complex queries, for example
//! filtering updates by release, status, security impact, reboot suggestion,
//! or for updates that are associated with a given set of packages.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::*;
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

const DESCRIPTION: &str = "description";
const NO_SUCH_UPDATE: &str = "Invalid update id";

/// Use this for querying bodhi for a specific update by its ID, title, or alias.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from(bodhi::FEDORA_BODHI_URL));
///
/// let update = bodhi::UpdateIDQuery::new(String::from("FEDORA-2019-3dd0cf468e"))
///     .query(&bodhi).unwrap();
/// ```
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
    /// This method is the only way to create a new `UpdateIDQuery` instance.
    pub fn new(id: String) -> UpdateIDQuery {
        UpdateIDQuery { id }
    }

    /// This method will query the remote bodhi instance for the requested update by ID,
    /// title, or alias, and will either return an `Ok(Some(Update))` matching the specified ID,
    /// title, or alias, return `Ok(None)` if it doesn't exist, or return an `Err(String)`
    /// if another error occurred.
    pub fn query(self, bodhi: &BodhiService) -> Result<Option<Update>, String> {
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

            Ok(Some(update.update))
        } else {
            let error: BodhiError = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("Unexpected error message: {:?}", error));
                }
            };

            // check if bodhi returned a "Invalid update id" error
            if !error.errors.is_empty() {
                let message = error
                    .errors
                    .get(0)
                    .expect("Despite a length greater 0, getting the first element failed.");

                if message.contains_key(DESCRIPTION) {
                    let description = message
                        .get(DESCRIPTION)
                        .expect("Despite the hash map containing the key, fetching value failed.");

                    if description == NO_SUCH_UPDATE {
                        // in this case, the query was successful, but nothing was found
                        return Ok(None);
                    }
                }
            }

            Err(format!("{:?}", error))
        }
    }
}

/// Use this for querying bodhi about a set of updates with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and updates will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from(bodhi::FEDORA_BODHI_URL));
///
/// let updates = bodhi::UpdateQuery::new()
///     .users(String::from("decathorpe"))
///     .releases(String::from("F30"))
///     .status(bodhi::UpdateStatus::Testing)
///     .query(&bodhi).unwrap();
/// ```
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
    /// This method returns a new `UpdateQuery` with *no* filters set.
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

    /// Restrict the returned results to (not) active releases.
    pub fn active_releases(mut self, active_releases: bool) -> UpdateQuery {
        self.active_releases = Some(active_releases);
        self
    }

    /// Restrict results to updates matching the given alias(es).
    /// Can be specified multiple times.
    pub fn aliases(mut self, alias: String) -> UpdateQuery {
        match &mut self.aliases {
            Some(aliases) => aliases.push(alias),
            None => self.aliases = Some(vec![alias]),
        }

        self
    }

    /// Restrict the returned results to updates which were approved
    /// before the given date and time.
    pub fn approved_before(mut self, approved_before: String) -> UpdateQuery {
        self.approved_before = Some(approved_before);
        self
    }

    /// Restrict the returned results to updates which were approved
    /// since the given date and time.
    pub fn approved_since(mut self, approved_since: String) -> UpdateQuery {
        self.approved_since = Some(approved_since);
        self
    }

    /// Restrict results to updates associated with the given bug(s).
    /// Can be specified multiple times.
    pub fn bugs(mut self, bug: String) -> UpdateQuery {
        match &mut self.bugs {
            Some(bugs) => bugs.push(bug),
            None => self.bugs = Some(vec![bug]),
        }

        self
    }

    /// Restrict results to updates associated with the given build(s).
    /// Can be specified multiple times.
    pub fn builds(mut self, build: String) -> UpdateQuery {
        match &mut self.builds {
            Some(builds) => builds.push(build),
            None => self.builds = Some(vec![build]),
        }

        self
    }

    /// Restrict the returned results to the given content type.
    pub fn content_type(mut self, content_type: ContentType) -> UpdateQuery {
        self.content_type = Some(content_type.into());
        self
    }

    /// Restrict the returned results to updates (not) marked with critpath.
    pub fn critpath(mut self, critpath: bool) -> UpdateQuery {
        self.critpath = Some(critpath);
        self
    }

    /// Restrict results to updates associated with the given CVE(s).
    /// Can be specified multiple times.
    pub fn cves(mut self, cve: String) -> UpdateQuery {
        match &mut self.cves {
            Some(cves) => cves.push(cve),
            None => self.cves = Some(vec![cve]),
        }

        self
    }

    /// Restrict search to updates *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: String) -> UpdateQuery {
        self.like = Some(like);
        self
    }

    /// Restrict the returned results to (not) locked updates.
    pub fn locked(mut self, locked: bool) -> UpdateQuery {
        self.locked = Some(locked);
        self
    }

    /// Restrict the returned results to updates which were modified
    /// before the given date and time.
    pub fn modified_before(mut self, modified_before: String) -> UpdateQuery {
        self.modified_before = Some(modified_before);
        self
    }

    /// Restrict the returned results to updates which were modified
    /// since the given date and time.
    pub fn modified_since(mut self, modified_since: String) -> UpdateQuery {
        self.modified_since = Some(modified_since);
        self
    }

    /// Restrict results to updates associated for the given package(s).
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> UpdateQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict the returned results to (not) pushed updates.
    pub fn pushed(mut self, pushed: bool) -> UpdateQuery {
        self.pushed = Some(pushed);
        self
    }

    /// Restrict the returned results to updates which were pushed
    /// before the given date and time.
    pub fn pushed_before(mut self, pushed_before: String) -> UpdateQuery {
        self.pushed_before = Some(pushed_before);
        self
    }

    /// Restrict the returned results to updates which were pushed
    /// since the given date and time.
    pub fn pushed_since(mut self, pushed_since: String) -> UpdateQuery {
        self.pushed_since = Some(pushed_since);
        self
    }

    /// Restrict results to updates for the given release(s).
    /// Can be specified multiple times.
    pub fn releases(mut self, release: String) -> UpdateQuery {
        match &mut self.releases {
            Some(releases) => releases.push(release),
            None => self.releases = Some(vec![release]),
        }

        self
    }

    /// Restrict the returned results to updates with the given request.
    pub fn request(mut self, request: UpdateRequest) -> UpdateQuery {
        self.request = Some(request.into());
        self
    }

    /// Restrict search to updates containing the given argument.
    pub fn search(mut self, search: String) -> UpdateQuery {
        self.search = Some(search);
        self
    }

    /// Restrict the returned results to updates with the given severity.
    pub fn severity(mut self, severity: UpdateSeverity) -> UpdateQuery {
        self.severity = Some(severity.into());
        self
    }

    /// Restrict the returned results to updates with the given status.
    pub fn status(mut self, status: UpdateStatus) -> UpdateQuery {
        self.status = Some(status.into());
        self
    }

    /// Restrict the returned results to updates which were submitted
    /// before the given date and time.
    pub fn submitted_before(mut self, submitted_before: String) -> UpdateQuery {
        self.submitted_before = Some(submitted_before);
        self
    }

    /// Restrict the returned results to updates which were submitted
    /// since the given date and time.
    pub fn submitted_since(mut self, submitted_since: String) -> UpdateQuery {
        self.submitted_since = Some(submitted_since);
        self
    }

    /// Restrict the returned results to updates with the given "suggest" value.
    pub fn suggest(mut self, suggest: UpdateSuggestion) -> UpdateQuery {
        self.suggest = Some(suggest.into());
        self
    }

    /// Restrict results to updates matching the given update ID(s).
    /// Can be specified multiple times.
    pub fn update_ids(mut self, update_id: String) -> UpdateQuery {
        match &mut self.update_ids {
            Some(update_ids) => update_ids.push(update_id),
            None => self.update_ids = Some(vec![update_id]),
        }

        self
    }

    /// Restrict results to updates matching the given update type.
    pub fn update_type(mut self, update_type: UpdateType) -> UpdateQuery {
        self.update_type = Some(update_type.into());
        self
    }

    /// Restrict results to updates associated with the given user(s).
    /// Can be specified multiple times.
    pub fn users(mut self, user: String) -> UpdateQuery {
        match &mut self.users {
            Some(users) => users.push(user),
            None => self.users = Some(vec![user]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
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
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
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

    page: u32,
    rows_per_page: u32,
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
