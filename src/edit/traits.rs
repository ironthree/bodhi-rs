use crate::error::QueryError;
use crate::BodhiService;

#[async_trait::async_trait]
pub trait Edit<'a, T> {
    async fn edit(&'a self, bodhi: &'a BodhiService) -> Result<T, QueryError>;
}
