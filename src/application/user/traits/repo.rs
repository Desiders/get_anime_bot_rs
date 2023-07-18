use crate::application::user::dto::{CreateUser, UpdateUserLanguageCode, UpdateUserShowNsfw};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait UserRepo {
    type CreateError;
    type UpdateLanguageCodeError;
    type UpdateShowNsfwError;

    async fn create(&mut self, user: CreateUser) -> Result<(), Self::CreateError>;

    async fn update_language_code(
        &mut self,
        user: UpdateUserLanguageCode,
    ) -> Result<(), Self::UpdateLanguageCodeError>;

    async fn update_show_nsfw(
        &mut self,
        user: UpdateUserShowNsfw,
    ) -> Result<(), Self::UpdateShowNsfwError>;
}
