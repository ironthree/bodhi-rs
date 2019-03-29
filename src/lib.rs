extern crate reqwest;
extern crate serde;

use std::collections::HashMap;

use reqwest::Response;

mod data;
use data::*;


pub struct BodhiService {
    url: String,
}


impl BodhiService {
    pub fn new(url: String) -> BodhiService {
        BodhiService { url }
    }

    fn request(&self, path: &String, args: Option<HashMap<&str, String>>) -> Result<Response, String> {
        let client = reqwest::Client::new();
        let url = format!("{}/{}", &self.url, path);

        let query: Vec<(&str, String)> = match args {
            Some(mut map) => { map.drain().collect() }
            None => { Vec::new() }
        };

        dbg!(&path);
        dbg!(&query);

        let mut response = match client.get(&url).query(&query).send() {
            Ok(response) => response,
            Err(error) => { return Err(format!("{:#?}", error)); }
        };

        if !response.status().is_success() {
            return Err(format!("{:#?}", response.text()));
        };

        return Ok(response);
    }

    pub fn query_build_by_nvr(&self, nvr: &String) -> Result<Option<Build>, String> {
        let path = format!("/builds/{}", nvr);

        let mut response = self.request(&path, None)?;

        let build: Option<Build> = match response.json() {
            Ok(value) => value,
            Err(error) => { return Err(format!("{:?}", error)); }
        };

        Ok(build)
    }

    pub fn query_build(
        &self,
        nvr: Option<String>,
        packages: Option<Vec<String>>,
        releases: Option<Vec<String>>,
        updates: Option<Vec<String>>,
        page: Option<i32>,
        rows_per_page: Option<i32>,
    ) -> Result<BuildListPage, String> {
        let path = String::from("/builds/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(nvr) = nvr {
            args.insert("nvr", nvr);
        }

        if let Some(packages) = packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(releases) = releases {
            args.insert("releases", releases.join(","));
        }

        if let Some(updates) = updates {
            args.insert("updates", updates.join(","));
        }

        if let Some(page) = page {
            args.insert("page", format!("{}", page));
        }

        if let Some(rpp) = rows_per_page {
            args.insert("rows_per_page", format!("{}", rpp));
        }

        let mut response = self.request(&path, Some(args))?;

        let builds: BuildListPage = match response.json() {
            Ok(value) => value,
            Err(error) => { return Err(format!("{:?}", error)); }
        };

        Ok(builds)
    }

    pub fn query_comment_by_id(&self, id: i32) -> Result<Option<Comment>, String> {
        let path = format!("/comments/{}", id);

        let mut response = self.request(&path, None)?;

        let comment: Comment = match response.json() {
            Ok(value) => value,
            Err(error) => { return Err(format!("{:?}", error)); }
        };

        Ok(Some(comment))
    }

    pub fn query_comment(
        &self,
        like: Option<String>,
        search: Option<String>,
        page: Option<i32>,
        rows_per_page: Option<i32>,
        updates: Option<Vec<String>>,
        packages: Option<Vec<String>>,
        user: Option<Vec<String>>,
        update_owner: Option<Vec<String>>,
        ignore_user: Option<Vec<String>>,
        anonymous: Option<bool>,
        since: Option<String>,
    ) -> Result<CommentListPage, String> {
        let path = String::from("/comments/");

        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(like) = like {
            args.insert("like", like);
        }

        if let Some(search) = search {
            args.insert("search", search);
        }

        if let Some(page) = page {
            args.insert("page", format!("{}", page));
        }

        if let Some(rpp) = rows_per_page {
            args.insert("rows_per_page", format!("{}", rpp));
        }

        if let Some(updates) = updates {
            args.insert("updates", updates.join(","));
        }

        if let Some(packages) = packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(user) = user {
            args.insert("releases", user.join(","));
        }

        if let Some(owner) = update_owner {
            args.insert("update_owner", owner.join(","));
        }

        if let Some(ignored) = ignore_user {
            args.insert("ignore_user", ignored.join(","));
        }

        if let Some(anon) = anonymous {
            args.insert("anonymous", format!("{}", anon));
        }

        if let Some(date) = since {
            args.insert("since", date);
        }

        let mut response = self.request(&path, Some(args))?;

        let comments: CommentListPage = match response.json() {
            Ok(value) => value,
            Err(error) => { return Err(format!("{:?}", error)); }
        };

        Ok(comments)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
