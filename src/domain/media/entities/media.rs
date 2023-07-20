use super::Genre;
use crate::domain::media::value_objects::MediaUrl;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Media {
    url: MediaUrl,
    genre: Genre,
}

impl Media {
    /// Creates a new media
    /// # Arguments
    /// * `url` - The url of the media
    /// * `genre` - The genre of the media
    pub fn new(url: impl Into<MediaUrl>, genre: Genre) -> Self {
        Self {
            url: url.into(),
            genre,
        }
    }

    /// Returns the url of the media
    pub const fn url(&self) -> &MediaUrl {
        &self.url
    }

    /// Returns the genre of the media
    pub const fn genre(&self) -> &Genre {
        &self.genre
    }
}
