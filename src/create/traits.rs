#![allow(missing_docs)]

use crate::error::QueryError;
use crate::BodhiService;

pub trait Create<T> {
    fn create(&self, bodhi: &BodhiService) -> Result<T, QueryError>;
}
