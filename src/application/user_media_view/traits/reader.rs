use crate::{
    application::{
        common::exceptions::{RepoError, RepoKind},
        user_media_view::{
            dto::{
                GetUserMediaViewById, GetUserMediaViewByMediaAgeRestriction,
                GetUserMediaViewByMediaGenre, GetUserMediaViewByMediaId,
                GetUserMediaViewByMediaSourceId, GetUserMediaViewByMediaType,
                GetUserMediaViewByUserId, GetUserMediaViewByUserTgId,
            },
            exceptions::UserMediaViewIdNotExist,
        },
    },
    domain::user_media_view::entities::UserMediaView as UserMediaViewEntity,
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait UserMediaViewReader {
    async fn get_by_id(
        &mut self,
        user_media_view: GetUserMediaViewById,
    ) -> Result<UserMediaViewEntity, RepoKind<UserMediaViewIdNotExist>>;

    async fn get_by_user_id(
        &mut self,
        user_media_view: GetUserMediaViewByUserId,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_media_id(
        &mut self,
        user_media_view: GetUserMediaViewByMediaId,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_user_tg_id(
        &mut self,
        user_media_view: GetUserMediaViewByUserTgId,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_media_genre(
        &mut self,
        user_media_view: GetUserMediaViewByMediaGenre,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_media_type(
        &mut self,
        user_media_view: GetUserMediaViewByMediaType,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_media_age_restriction(
        &mut self,
        user_media_view: GetUserMediaViewByMediaAgeRestriction,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_media_source_id(
        &mut self,
        user_media_view: GetUserMediaViewByMediaSourceId,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;
}
