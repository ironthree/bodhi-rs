use crate::error::QueryError;
use crate::service::BodhiService;

pub trait Create<T> {
    fn create(&self, bodhi: &BodhiService) -> Result<T, QueryError>;
}
