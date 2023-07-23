use crate::{
    application::{
        common::exceptions::{RepoError, RepoKind},
        media::{
            dto::{GetMediaById, GetMediaByUrl},
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
}
