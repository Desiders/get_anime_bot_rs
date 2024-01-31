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
    async fn get_by_id<'s>(
        &mut self,
        user_media_view: GetUserMediaViewById<'s>,
    ) -> Result<UserMediaViewEntity, RepoKind<UserMediaViewIdNotExist>>;

    async fn get_by_user_id<'s>(
        &mut self,
        user_media_view: GetUserMediaViewByUserId<'s>,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_media_id<'s>(
        &mut self,
        user_media_view: GetUserMediaViewByMediaId<'s>,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_user_tg_id(
        &mut self,
        user_media_view: GetUserMediaViewByUserTgId,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_media_genre<'s>(
        &mut self,
        user_media_view: GetUserMediaViewByMediaGenre<'s>,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_media_type<'s>(
        &mut self,
        user_media_view: GetUserMediaViewByMediaType<'s>,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_media_age_restriction(
        &mut self,
        user_media_view: GetUserMediaViewByMediaAgeRestriction,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;

    async fn get_by_media_source_id<'s>(
        &mut self,
        user_media_view: GetUserMediaViewByMediaSourceId<'s>,
    ) -> Result<Vec<UserMediaViewEntity>, RepoError>;
}
