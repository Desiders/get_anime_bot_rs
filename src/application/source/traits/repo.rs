use crate::application::source::dto::CreateSource;

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait SourceRepo {
    type CreateError;

    async fn create(&mut self, source: CreateSource) -> Result<(), Self::CreateError>;
}
