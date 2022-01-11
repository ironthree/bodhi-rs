use crate::error::QueryError;

use serde::de::DeserializeOwned;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    GET,
    POST,
}

pub trait SingleRequest<P, T>
where
    T: DeserializeOwned,
{
    fn method(&self) -> RequestMethod;
    fn path(&self) -> Result<String, QueryError>;

    #[allow(unused_variables)]
    fn body(&self, csrf_token: Option<String>) -> Result<Option<String>, QueryError> {
        Ok(None)
    }

    fn parse(&self, string: &str) -> Result<P, QueryError>;
    fn extract(&self, page: P) -> T;
}

pub trait PaginatedRequest<P, T>
where
    P: Pagination,
    T: DeserializeOwned,
{
    fn page_request(&self, page: u32) -> Box<dyn SingleRequest<P, T>>;
    fn callback(&self, page: u32, pages: u32);
}

pub trait Pagination {
    fn pages(&self) -> u32;
}
