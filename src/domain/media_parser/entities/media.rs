use super::Genre;
use crate::domain::media_parser::value_objects::MediaUrl;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Media {
    url: Cow<'static, MediaUrl>,
    genre: Genre,
}

impl Media {
    /// Creates a new media
    /// # Arguments
    /// * `url` - The url of the media
    /// * `genre` - The genre of the media
    pub fn new(url: impl Into<Cow<'static, MediaUrl>>, genre: Genre) -> Self {
        Self {
            url: url.into(),
            genre,
        }
    }

    /// Returns the url of the media
    pub fn url(&self) -> &MediaUrl {
        &self.url
    }

    /// Returns the genre of the media
    pub const fn genre(&self) -> &Genre {
        &self.genre
    }
}
