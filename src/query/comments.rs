use std::collections::HashMap;

use crate::data::{Comment, CommentPage, CommentListPage};
use crate::service::BodhiService;


#[derive(Debug, Default)]
pub struct CommentIDQuery {
    id: i32,
    anonymous: bool,
}


impl CommentIDQuery {
    pub fn new(id: i32) -> CommentIDQuery {
        CommentIDQuery { id, anonymous: false }
    }

    pub fn anonymous(mut self, anonymous: bool) -> CommentIDQuery {
        self.anonymous = anonymous;
        self
    }

    pub fn query(self, bodhi: &BodhiService) -> Result<Option<Comment>, String> {
        let path = format!("/comments/{}", self.id);

        let mut args: HashMap<&str, String> = HashMap::new();
        args.insert("anonymous", format!("{}", self.anonymous));

        let mut response = bodhi.request(&path, Some(args))?;

        let comment: CommentPage = match response.json() {
            Ok(value) => value,
            Err(error) => { return Err(format!("{:?}", error)); }
        };

        Ok(Some(comment.comment))
    }
}


#[derive(Debug, Default)]
pub struct CommentQuery {
    like: Option<String>,
    search: Option<String>,
    updates: Option<Vec<String>>,
    packages: Option<Vec<String>>,
    users: Option<Vec<String>>,
    owners: Option<Vec<String>>,
    ignore_users: Option<Vec<String>>,
    anonymous: bool,
    since: Option<String>,
}


impl CommentQuery {
    pub fn new() -> CommentQuery {
        CommentQuery {
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

    pub fn like(mut self, like: String) -> CommentQuery {
        self.like = Some(like);
        self
    }

    pub fn search(mut self, search: String) -> CommentQuery {
        self.search = Some(search);
        self
    }

    pub fn update(mut self, update: String) -> CommentQuery {
        match &mut self.updates {
            Some(updates) => updates.push(update),
            None => self.updates = Some(vec!(update)),
        }

        self
    }

    pub fn package(mut self, package: String) -> CommentQuery {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec!(package)),
        }

        self
    }

    pub fn user(mut self, user: String) -> CommentQuery {
        match &mut self.users {
            Some(users) => users.push(user),
            None => self.users = Some(vec!(user)),
        }

        self
    }

    pub fn owner(mut self, owner: String) -> CommentQuery {
        match &mut self.owners {
            Some(owners) => owners.push(owner),
            None => self.owners = Some(vec!(owner)),
        }

        self
    }

    pub fn ignore_user(mut self, owner: String) -> CommentQuery {
        match &mut self.ignore_users {
            Some(ignored) => ignored.push(owner),
            None => self.ignore_users = Some(vec!(owner)),
        }

        self
    }

    pub fn anonymous(mut self, anonymous: bool) -> CommentQuery {
        self.anonymous = anonymous;
        self
    }

    pub fn since(mut self, since: String) -> CommentQuery {
        self.since = Some(since);
        self
    }

    // TODO: query all pages and return the union
    // TODO: right now, only the first page (20 items) is returned
    pub fn query(self, bodhi: &BodhiService) -> Result<CommentListPage, String> {
        let path = String::from("/comments/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(like) = self.like {
            args.insert("like", like);
        }

        if let Some(search) = self.search {
            args.insert("search", search);
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
