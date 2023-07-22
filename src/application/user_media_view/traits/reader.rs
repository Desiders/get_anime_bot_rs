use crate::{
    application::user_media_view::dto::{
        GetUserMediaViewById, GetUserMediaViewByMediaId, GetUserMediaViewByUserId,
    },
    domain::user_media_view::entities::UserMediaView as UserMediaViewEntity,
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait UserMediaViewReader {
    type GetError;
    type GetByIdError;
    type GetByUserIdError;
    type GetByMediaIdError;

    async fn get_by_id(
        &mut self,
        user_media_view: GetUserMediaViewById,
    ) -> Result<UserMediaViewEntity, Self::GetError>;

    async fn get_by_user_id(
        &mut self,
        user_media_view: GetUserMediaViewByUserId,
    ) -> Result<Vec<UserMediaViewEntity>, Self::GetByUserIdError>;

    async fn get_by_media_id(
        &mut self,
        user_media_view: GetUserMediaViewByMediaId,
    ) -> Result<Vec<UserMediaViewEntity>, Self::GetByMediaIdError>;
}
