use crate::application::{
    common::exceptions::RepoKind,
    media::{dto::CreateMedia, exceptions::MediaUrlAndGenreAlreadyExists},
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait MediaRepo {
    async fn create(
        &mut self,
        media: CreateMedia,
    ) -> Result<(), RepoKind<MediaUrlAndGenreAlreadyExists>>;
}
