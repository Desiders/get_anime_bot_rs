use crate::{
    application::{
        common::exceptions::{RepoError, RepoKind},
        media::{
            dto::{GetMediaById, GetMediaByInfo, GetMediaByInfoUnviewedByUser, GetMediaByUrl},
            exceptions::MediaIdNotExist,
        },
    },
    domain::media::entities::Media as MediaEntity,
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait MediaReader {
    async fn get_by_id(
        &mut self,
        media: GetMediaById,
    ) -> Result<MediaEntity, RepoKind<MediaIdNotExist>>;

    async fn get_by_url(&mut self, media: GetMediaByUrl) -> Result<Vec<MediaEntity>, RepoError>;

    async fn get_by_info(&mut self, media: GetMediaByInfo) -> Result<Vec<MediaEntity>, RepoError>;

    async fn get_by_info_unviewed_by_user(
        &mut self,
        media: GetMediaByInfoUnviewedByUser,
    ) -> Result<Vec<MediaEntity>, RepoError>;
}
