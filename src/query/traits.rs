use crate::error::{BodhiError, QueryError};
use crate::BodhiService;

#[async_trait::async_trait]
pub trait Query<'a, T> {
    async fn query(&'a self, bodhi: &'a BodhiService) -> Result<T, QueryError>;
}

#[async_trait::async_trait]
pub trait SinglePageQuery<T> {
    /// This method is expected to return the path of the API endpoint.
    fn path(&self) -> Result<String, QueryError>;

    /// This associated method is expected to return the result that was parsed from the JSON
    /// response, or an error.
    fn parse(string: &str) -> Result<T, QueryError>;

    /// This associated method returns the item that represents the "missing" state (when getting a
    /// 404 error from the server).
    ///
    /// This can be a valid response for missing things (in which case the trait implementation will
    /// probably will return `None` here, or an invalid response, where the trait implementation
    /// will return an error.
    fn missing() -> Result<T, QueryError>;

    /// This method executes a single-page query, but delegates execution of some things to the
    /// individual trait implementations (such as deserializing JSON, handling 404 errors, or
    /// getting API paths and arguments).
    async fn query(self, bodhi: &BodhiService) -> Result<T, QueryError>
    where
        Self: Sized,
    {
        let path = self.path()?;
        let response = bodhi.get(&path).await?;
        let status = response.status();

        if status.is_success() {
            let string = response.text().await?;
            <Self as SinglePageQuery<T>>::parse(&string)
        } else if status == 404 {
            <Self as SinglePageQuery<T>>::missing()
        } else {
            let result = response.text().await?;
            let error: BodhiError = serde_json::from_str(&result)?;

            Err(QueryError::BodhiError { error })
        }
    }
}
