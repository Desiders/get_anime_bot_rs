use crate::application::user_media_view::dto::CreateUserMediaView;

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait UserMediaViewRepo {
    type CreateError;

    async fn create(
        &mut self,
        user_media_view: CreateUserMediaView,
    ) -> Result<(), Self::CreateError>;
}
