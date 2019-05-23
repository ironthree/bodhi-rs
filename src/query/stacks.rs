//! query for stacks (or *one* stack by name)
//!
//! The contents of this module can be used to query a bodhi instance about
//! existing stacks.
//!
//! The `StackNameQuery` returns exactly one Stack, if and only if a Stack
//! with this name exists - otherwise, it will return an error.
//!
//! The `StackQuery` can be used to execute more complex queries, for example
//! filtering stacks that are associated with a given set of packages.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Stack};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

const DESCRIPTION: &str = "description";
const NO_SUCH_STACK: &str = "Invalid stack specified:";

/// Use this for querying bodhi for a specific stack by its name.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from(bodhi::FEDORA_BODHI_URL));
///
/// let comment = bodhi::StackNameQuery::new(String::from("SomeStack"))
///     .query(&bodhi).unwrap();
/// ```
#[derive(Debug)]
pub struct StackNameQuery {
    name: String,
}

#[derive(Debug, Deserialize)]
struct StackPage {
    stack: Stack,
}

impl StackNameQuery {
    /// This method is the only way to create a new `StackNameQuery` instance.
    pub fn new(name: String) -> StackNameQuery {
        StackNameQuery { name }
    }

    /// This method will query the remote bodhi instance for the requested stack by name,
    /// and will either return an `Ok(Some(Stack))` matching the specified name,
    /// return `Ok(None)` if it doesn't exist, or return an `Err(String)`
    /// if another error occurred.
    pub fn query(self, bodhi: &BodhiService) -> Result<Option<Stack>, String> {
        let path = format!("/stacks/{}", self.name);

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let stack: StackPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(Some(stack.stack))
        } else {
            let error: BodhiError = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("Unexpected error message: {:?}", error));
                }
            };

            // check if bodhi returned a "Invalid stack specified:" error
            if !error.errors.is_empty() {
                let message = error
                    .errors
                    .get(0)
                    .expect("Despite a length greater 0, getting the first element failed.");

                if message.contains_key(DESCRIPTION) {
                    let description = message
                        .get(DESCRIPTION)
                        .expect("Despite the hash map containing the key, fetching value failed.");

                    if description.starts_with(NO_SUCH_STACK) {
                        // in this case, the query was successful, but nothing was found
                        return Ok(None);
                    }
                }
            }

            Err(format!("{:?}", error))
        }
    }
}

/// Use this for querying bodhi about a set of stacks with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and stacks will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiService::new(String::from(bodhi::FEDORA_BODHI_URL));
///
/// let comments = bodhi::StackQuery::new().query(&bodhi).unwrap();
/// ```
#[derive(Debug, Default)]
pub struct StackQuery {
    like: Option<String>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
}

impl StackQuery {
    /// This method returns a new `StackQuery` with *no* filters set.
    pub fn new() -> StackQuery {
        StackQuery {
            like: None,
            name: None,
            packages: None,
            search: None,
        }
    }

    /// Restrict search to stacks *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: String) -> StackQuery {
        self.like = Some(like);
        self
    }

    /// Restrict results to stacks with the given name.
    /// If this is the only required filter, consider using a `StackNameQuery` instead.
    pub fn name(mut self, name: String) -> StackQuery {
        self.name = Some(name);
        self
    }

    /// Restrict the returned results to stacks containing the given package(s).
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> StackQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict search to stacks containing the given argument.
    pub fn search(mut self, search: String) -> StackQuery {
        self.search = Some(search);
        self
    }

    /// Query the remote bodhi instance with the given parameters.
    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Stack>, String> {
        let mut stacks: Vec<Stack> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = StackPageQuery::new();
            query.page = page;

            query.like = self.like.clone();
            query.name = self.name.clone();
            query.search = self.search.clone();
            query.packages = self.packages.clone();

            let result = query.query(bodhi)?;
            stacks.extend(result.stacks);

            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(stacks)
    }
}

#[derive(Debug, Deserialize)]
struct StackListPage {
    stacks: Vec<Stack>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

#[derive(Debug)]
struct StackPageQuery {
    like: Option<String>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,

    page: u32,
    rows_per_page: u32,
}

impl StackPageQuery {
    fn new() -> StackPageQuery {
        StackPageQuery {
            like: None,
            name: None,
            packages: None,
            search: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
        }
    }

    fn query(self, bodhi: &BodhiService) -> Result<StackListPage, String> {
        let path = String::from("/stacks/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(like) = self.like {
            args.insert("like", like);
        };

        if let Some(name) = self.name {
            args.insert("name", name);
        };

        if let Some(packages) = self.packages {
            args.insert("packages", packages.join(","));
        };

        if let Some(search) = self.search {
            args.insert("search", search);
        };

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        let mut response = bodhi.request(&path, Some(args))?;
        let status = response.status();

        if status.is_success() {
            let stacks: StackListPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(stacks)
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
