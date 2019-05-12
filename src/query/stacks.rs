use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Stack};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

#[derive(Debug)]
pub struct StackNameQuery {
    name: String,
}

#[derive(Debug, Deserialize)]
struct StackPage {
    stack: Stack,
}

impl StackNameQuery {
    pub fn new(name: String) -> StackNameQuery {
        StackNameQuery { name }
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Stack, String> {
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

            Ok(stack.stack)
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
pub struct StackQuery {
    like: Option<String>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
}

impl StackQuery {
    pub fn new() -> StackQuery {
        StackQuery {
            like: None,
            name: None,
            packages: None,
            search: None,
        }
    }

    pub fn like(mut self, like: String) -> StackQuery {
        self.like = Some(like);
        self
    }

    pub fn name(mut self, name: String) -> StackQuery {
        self.name = Some(name);
        self
    }

    pub fn packages(mut self, package: String) -> StackQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    pub fn search(mut self, search: String) -> StackQuery {
        self.search = Some(search);
        self
    }

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
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}

#[derive(Debug)]
struct StackPageQuery {
    like: Option<String>,
    name: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,

    page: i32,
    rows_per_page: i32,
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
