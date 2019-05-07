use serde::Deserialize;

use crate::data::{BodhiError, Comment};
use crate::service::BodhiService;

//const DEFAULT_PAGE: i32 = 1;
//const DEFAULT_ROWS: i32 = 50;

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

/*
#[derive(Debug)]
pub struct CommentQuery {
    // TODO
}

impl CommentQuery {
    pub fn new() -> CommentQuery {
        CommentQuery {
            // TODO
        }
    }

    // TODO: query modifiers

    pub fn nvr(mut self, nvr: String) -> CommentQuery {
        self.nvr = Some(nvr);
        self
    }

    pub fn package(mut self, package: String) -> CommentQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Vec<Comment>, String> {
        let mut comments: Vec<Comment> = Vec::new();
        let mut page = 1;

        loop {
            let query = CommentPageQuery {
                // TODO
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
    pub like: Option<String>,
    pub search: Option<String>,
    pub updates: Option<Vec<String>>,
    pub packages: Option<Vec<String>>,
    pub users: Option<Vec<String>>,
    pub owners: Option<Vec<String>>,
    pub ignore_users: Option<Vec<String>>,
    pub anonymous: bool,
    pub since: Option<String>,
}

impl CommentPageQuery {
    fn new() -> CommentPageQuery {
        CommentPageQuery {
            like: None,
            search: None,
            updates: None,
            packages: None,
            users: None,
            owners: None,
            ignore_users: None,
            anonymous: false,
            since: None,
        }
    }

    fn like(mut self, like: String) -> CommentPageQuery {
        self.like = Some(like);
        self
    }

    fn search(mut self, search: String) -> CommentPageQuery {
        self.search = Some(search);
        self
    }

    fn update(mut self, update: String) -> CommentPageQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec!(update)),
        }

        self
    }

    fn package(mut self, package: String) -> CommentPageQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec!(package)),
        }

        self
    }

    fn user(mut self, user: String) -> CommentPageQuery {
        match &mut self.users {
            Some(users) => users.push(user),
            None => self.users = Some(vec!(user)),
        }

        self
    }

    fn owner(mut self, owner: String) -> CommentPageQuery {
        match &mut self.owners {
            Some(owners) => owners.push(owner),
            None => self.owners = Some(vec!(owner)),
        }

        self
    }

    fn ignore_user(mut self, owner: String) -> CommentPageQuery {
        match &mut self.ignore_users {
            Some(ignored) => ignored.push(owner),
            None => self.ignore_users = Some(vec!(owner)),
        }

        self
    }

    fn anonymous(mut self, anonymous: bool) -> CommentPageQuery {
        self.anonymous = anonymous;
        self
    }

    fn since(mut self, since: String) -> CommentPageQuery {
        self.since = Some(since);
        self
    }

    // TODO: query all pages and return the union
    // TODO: right now, only the first page (20 items) is returned
    fn query(self, bodhi: &BodhiService) -> Result<CommentListPage, String> {
        let path = String::from("/comments/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(like) = self.like {
            args.insert("like", like);
        }

        if let Some(search) = self.search {
            args.insert("search", search);
        }

        // TODO: first check the number of items and then retrieve all of them
        if let Some(page) = page {
            args.insert("page", format!("{}", page));
        }

        if let Some(rpp) = rows_per_page {
            args.insert("rows_per_page", format!("{}", rpp));
        }

        if let Some(updates) = self.updates {
            args.insert("updates", updates.join(","));
        }

        if let Some(packages) = self.packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(user) = self.users {
            args.insert("releases", user.join(","));
        }

        if let Some(owner) = self.owners {
            args.insert("update_owner", owner.join(","));
        }

        if let Some(ignored) = self.ignore_users {
            args.insert("ignore_user", ignored.join(","));
        }

        args.insert("anonymous", format!("{}", self.anonymous));

        if let Some(date) = self.since {
            args.insert("since", date);
        }

        let mut response = bodhi.request(&path, Some(args))?;

        let comments: CommentListPage = match response.json() {
            Ok(value) => value,
            Err(error) => { return Err(format!("{:?}", error)); }
        };

        Ok(comments)
    }
}
*/
