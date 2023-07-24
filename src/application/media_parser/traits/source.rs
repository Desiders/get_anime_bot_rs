use crate::{
    application::media_parser::exceptions::MediaGetException,
    domain::media_parser::entities::{Genre, Genres, Media},
};

use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Source: Send + Sync {
    /// Get the genres of the media source
    fn genres(&self) -> &Genres;

    /// Get a list of media by genre
    /// # Arguments
    /// * `genre` - The genre to get the media from
    /// # Returns
    /// [`Vec<Media>`] if the request was successful, [`MediaSource::GetMediaError`] otherwise
    async fn get_media_list_by_genre(&self, genre: &Genre)
        -> Result<Vec<Media>, MediaGetException>;
}

#[async_trait]
impl<S> Source for Arc<S>
where
    S: Source + ?Sized,
{
    fn genres(&self) -> &Genres {
        (**self).genres()
    }

    async fn get_media_list_by_genre(
        &self,
        genre: &Genre,
    ) -> Result<Vec<Media>, MediaGetException> {
        (**self).get_media_list_by_genre(genre).await
    }
}
