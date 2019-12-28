use crate::error::QueryError;
use crate::BodhiService;

pub trait Edit<T> {
    fn edit(&self, bodhi: &BodhiService) -> Result<T, QueryError>;
}
