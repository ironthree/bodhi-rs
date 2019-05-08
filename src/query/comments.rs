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
    pub comment: Comment,
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
    ignore_user: Option<Vec<String>>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    search: Option<String>,
    // TODO: since: Option<DateTimeString>,
    update_owner: Option<Vec<String>>,
    updates: Option<Vec<String>>,
    user: Option<Vec<String>>,
}

impl CommentQuery {
    pub fn new() -> CommentQuery {
        CommentQuery {
            anonymous: None,
            ignore_user: None,
            like: None,
            packages: None,
            search: None,
            update_owner: None,
            updates: None,
            user: None,
        }
    }

    pub fn anonymous(mut self, anonymous: bool) -> CommentQuery {
        self.anonymous = Some(anonymous);
        self
    }

    pub fn ignore_user(mut self, ignore_user: String) -> CommentQuery {
        match &mut self.ignore_user {
            Some(ignore_users) => ignore_users.push(ignore_user),
            None => self.ignore_user = Some(vec![ignore_user]),
        }

        self
    }

    pub fn like(mut self, like: String) -> CommentQuery {
        self.like = Some(like);
        self
    }

    pub fn package(mut self, package: String) -> CommentQuery {
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

    pub fn update_owner(mut self, update_owner: String) -> CommentQuery {
        match &mut self.update_owner {
            Some(update_owners) => update_owners.push(update_owner),
            None => self.update_owner = Some(vec![update_owner]),
        }

        self
    }

    pub fn update(mut self, update: String) -> CommentQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    pub fn user(mut self, user: String) -> CommentQuery {
        match &mut self.user {
            Some(users) => users.push(user),
            None => self.user = Some(vec![user]),
        }

        self
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Comment>, String> {
        let mut comments: Vec<Comment> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = CommentPageQuery::new().page(page);

            if let Some(anonymous) = self.anonymous {
                query = query.anonymous(anonymous);
            };

            if let Some(ignore_users) = self.ignore_user.clone() {
                for ignore_user in ignore_users {
                    query = query.ignore_user(ignore_user);
                }
            };

            if let Some(like) = self.like.clone() {
                query = query.like(like);
            };

            if let Some(packages) = self.packages.clone() {
                for package in packages {
                    query = query.package(package);
                }
            };

            if let Some(search) = self.search.clone() {
                query = query.search(search);
            };

            if let Some(update_owners) = self.update_owner.clone() {
                for update_owner in update_owners {
                    query = query.update_owner(update_owner);
                }
            };

            if let Some(updates) = self.updates.clone() {
                for update in updates {
                    query = query.update(update);
                }
            };

            if let Some(users) = self.user.clone() {
                for user in users {
                    query = query.user(user);
                }
            };

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
    pub comments: Vec<Comment>,
    pub page: i32,
    pub pages: i32,
    pub rows_per_page: i32,
    pub total: i32,
}

#[derive(Debug)]
struct CommentPageQuery {
    anonymous: Option<bool>,
    ignore_user: Option<Vec<String>>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    page: i32,
    rows_per_page: i32,
    search: Option<String>,
    // TODO: since: Option<DateTimeString>,
    update_owner: Option<Vec<String>>,
    updates: Option<Vec<String>>,
    user: Option<Vec<String>>,
}

impl CommentPageQuery {
    fn new() -> CommentPageQuery {
        CommentPageQuery {
            anonymous: None,
            ignore_user: None,
            like: None,
            packages: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
            search: None,
            update_owner: None,
            updates: None,
            user: None,
        }
    }

    fn anonymous(mut self, anonymous: bool) -> CommentPageQuery {
        self.anonymous = Some(anonymous);
        self
    }

    fn ignore_user(mut self, ignore_user: String) -> CommentPageQuery {
        match &mut self.ignore_user {
            Some(ignore_users) => ignore_users.push(ignore_user),
            None => self.ignore_user = Some(vec![ignore_user]),
        }

        self
    }

    fn like(mut self, like: String) -> CommentPageQuery {
        self.like = Some(like);
        self
    }

    fn package(mut self, package: String) -> CommentPageQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    fn search(mut self, search: String) -> CommentPageQuery {
        self.search = Some(search);
        self
    }

    fn update_owner(mut self, update_owner: String) -> CommentPageQuery {
        match &mut self.update_owner {
            Some(update_owners) => update_owners.push(update_owner),
            None => self.update_owner = Some(vec![update_owner]),
        }

        self
    }

    fn update(mut self, update: String) -> CommentPageQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec![update]),
        }

        self
    }

    fn user(mut self, user: String) -> CommentPageQuery {
        match &mut self.user {
            Some(users) => users.push(user),
            None => self.user = Some(vec![user]),
        }

        self
    }

    fn page(mut self, page: i32) -> CommentPageQuery {
        self.page = page;
        self
    }

    /*
    fn rows_per_page(mut self, rows_per_page: i32) -> CommentPageQuery {
        self.rows_per_page = rows_per_page;
        self
    }
    */

    fn query(self, bodhi: &BodhiService) -> Result<CommentListPage, String> {
        let path = String::from("/comments/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(anonymous) = self.anonymous {
            args.insert("anonymous", anonymous.to_string());
        }

        if let Some(ignore_users) = self.ignore_user {
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

        if let Some(update_owners) = self.update_owner {
            args.insert("update_owner", update_owners.join(","));
        }

        if let Some(updates) = self.updates {
            args.insert("updates", updates.join(","));
        }

        if let Some(users) = self.user {
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
