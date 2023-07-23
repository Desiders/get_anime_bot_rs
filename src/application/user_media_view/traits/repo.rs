use crate::application::{
    common::exceptions::RepoKind,
    user_media_view::{
        dto::CreateUserMediaView, exceptions::UserMediaViewUserIdAndMediaIdAlreadyExists,
    },
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait UserMediaViewRepo {
    async fn create(
        &mut self,
        user_media_view: CreateUserMediaView,
    ) -> Result<(), RepoKind<UserMediaViewUserIdAndMediaIdAlreadyExists>>;
}
