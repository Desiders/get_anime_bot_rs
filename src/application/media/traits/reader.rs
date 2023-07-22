use crate::{
    application::media::dto::{GetMediaById, GetMediaByUrl},
    domain::media::entities::Media as MediaEntity,
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait MediaReader {
    type GetError;
    type GetByIdError;

    async fn get_by_id(&mut self, media: GetMediaById) -> Result<MediaEntity, Self::GetError>;

    async fn get_by_url(&mut self, media: GetMediaByUrl)
        -> Result<MediaEntity, Self::GetByIdError>;
}
