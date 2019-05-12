use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{BodhiError, Comment};
use crate::service::{BodhiService, DEFAULT_PAGE, DEFAULT_ROWS};

#[derive(Debug)]
pub struct CommentIDQuery {
    id: i32,
}

#[derive(Debug, Deserialize)]
struct CommentPage {
    comment: Comment,
}

impl CommentIDQuery {
    pub fn new(id: i32) -> CommentIDQuery {
        CommentIDQuery { id }
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Comment, String> {
        let path = format!("/comments/{}", self.id);

        let mut response = bodhi.request(&path, None)?;
        let status = response.status();

        if status.is_success() {
            let comment: CommentPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(comment.comment)
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
pub struct CommentQuery {
    anonymous: Option<bool>,
    ignore_users: Option<Vec<String>>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
    since: Option<String>,
    update_owners: Option<Vec<String>>,
    updates: Option<Vec<String>>,
    users: Option<Vec<String>>,
}

impl CommentQuery {
    pub fn new() -> CommentQuery {
        CommentQuery {
            anonymous: None,
            ignore_users: None,
            like: None,
            packages: None,
            search: None,
            since: None,
            update_owners: None,
            updates: None,
            users: None,
        }
    }

    pub fn anonymous(mut self, anonymous: bool) -> CommentQuery {
        self.anonymous = Some(anonymous);
        self
    }

    pub fn ignore_users(mut self, ignore_user: String) -> CommentQuery {
        match &mut self.ignore_users {
            Some(ignore_users) => ignore_users.push(ignore_user),
            None => self.ignore_users = Some(vec![ignore_user]),
        }

        self
    }

    pub fn like(mut self, like: String) -> CommentQuery {
        self.like = Some(like);
        self
    }

    pub fn packages(mut self, package: String) -> CommentQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    pub fn search(mut self, search: String) -> CommentQuery {
        self.search = Some(search);
        self
    }

    pub fn since(mut self, since: String) -> CommentQuery {
        self.since = Some(since);
        self
    }

    pub fn update_owners(mut self, update_owner: String) -> CommentQuery {
        match &mut self.update_owners {
            Some(update_owners) => update_owners.push(update_owner),
            None => self.update_owners = Some(vec![update_owner]),
        }

        self
    }

    pub fn updates(mut self, update: String) -> CommentQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    pub fn users(mut self, user: String) -> CommentQuery {
        match &mut self.users {
            Some(users) => users.push(user),
            None => self.users = Some(vec![user]),
        }

        self
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Comment>, String> {
        let mut comments: Vec<Comment> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = CommentPageQuery::new();
            query.page = page;

            query.anonymous = self.anonymous;
            query.ignore_users = self.ignore_users.clone();
            query.like = self.like.clone();
            query.packages = self.packages.clone();
            query.search = self.search.clone();
            query.update_owners = self.update_owners.clone();
            query.updates = self.updates.clone();
            query.users = self.users.clone();

            let result = query.query(bodhi)?;
            comments.extend(result.comments);

            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(comments)
    }
}

#[derive(Debug, Deserialize)]
struct CommentListPage {
    comments: Vec<Comment>,
    page: i32,
    pages: i32,
    rows_per_page: i32,
    total: i32,
}

#[derive(Debug)]
struct CommentPageQuery {
    anonymous: Option<bool>,
    ignore_users: Option<Vec<String>>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
    since: Option<String>,
    update_owners: Option<Vec<String>>,
    updates: Option<Vec<String>>,
    users: Option<Vec<String>>,

    page: i32,
    rows_per_page: i32,
}

impl CommentPageQuery {
    fn new() -> CommentPageQuery {
        CommentPageQuery {
            anonymous: None,
            ignore_users: None,
            like: None,
            packages: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
            search: None,
            since: None,
            update_owners: None,
            updates: None,
            users: None,
        }
    }

    fn query(self, bodhi: &BodhiService) -> Result<CommentListPage, String> {
        let path = String::from("/comments/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(anonymous) = self.anonymous {
            args.insert("anonymous", anonymous.to_string());
        }

        if let Some(ignore_users) = self.ignore_users {
            args.insert("ignore_user", ignore_users.join(","));
        }

        if let Some(like) = self.like {
            args.insert("like", like);
        }

        if let Some(packages) = self.packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(search) = self.search {
            args.insert("search", search);
        }

        if let Some(update_owners) = self.update_owners {
            args.insert("update_owner", update_owners.join(","));
        }

        if let Some(updates) = self.updates {
            args.insert("updates", updates.join(","));
        }

        if let Some(users) = self.users {
            args.insert("user", users.join(","));
        }

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        let mut response = bodhi.request(&path, Some(args))?;
        let status = response.status();

        if status.is_success() {
            let comments: CommentListPage = match response.json() {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!("{:?}", error));
                }
            };

            Ok(comments)
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
