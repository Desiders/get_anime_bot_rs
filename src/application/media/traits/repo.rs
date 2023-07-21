use crate::application::media::dto::CreateMedia;

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait MediaRepo {
    type CreateError;

    async fn create(&mut self, media: CreateMedia) -> Result<(), Self::CreateError>;
}
