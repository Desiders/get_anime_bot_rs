use crate::{
    application::{
        common::exceptions::{RepoError, RepoKind},
        media::{
            dto::{GetMediaById, GetMediaByInfo, GetMediaByInfoUnviewedByUser, GetMediaByUrl},
            exceptions::MediaIdNotExist,
        },
    },
    domain::media::entities::{GenresStats, Media as MediaEntity, MediaStats},
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait MediaReader {
    async fn get_by_id<'s>(
        &mut self,
        media: GetMediaById<'s>,
    ) -> Result<MediaEntity, RepoKind<MediaIdNotExist>>;

    async fn get_by_url<'s>(
        &mut self,
        media: GetMediaByUrl<'s>,
    ) -> Result<Vec<MediaEntity>, RepoError>;

    async fn get_by_info<'s>(
        &mut self,
        media: GetMediaByInfo<'s>,
    ) -> Result<Vec<MediaEntity>, RepoError>;

    async fn get_by_info_unviewed_by_user<'s>(
        &mut self,
        media: GetMediaByInfoUnviewedByUser<'s>,
    ) -> Result<Vec<MediaEntity>, RepoError>;

    async fn get_media_stats(&mut self) -> Result<MediaStats, RepoError>;

    async fn get_genre_stats(&mut self) -> Result<GenresStats, RepoError>;
}
