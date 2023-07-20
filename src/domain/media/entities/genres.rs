use super::Genre;
use crate::domain::media::value_objects::{AgeRestriction, MediaType};

use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Genres {
    inner: Vec<Genre>,
}

impl Genres {
    /// Creates a new genres
    /// # Arguments
    /// * `inner` - The inner genres
    pub fn new(inner: Vec<Genre>) -> Self {
        Self { inner }
    }
}

impl Genres {
    /// Filters the genres
    /// # Arguments
    /// * `media_type` - The media type to filter. If `None`, it will not filter by media type
    /// * `age_restriction` - The age restriction to filter. If `None`, it will not filter by age restriction
    /// # Returns
    /// The filtered genres
    pub fn filter(
        &self,
        media_type: Option<MediaType>,
        age_restriction: Option<AgeRestriction>,
    ) -> Vec<Genre> {
        let genres: Vec<&Genre> = if let Some(media_type) = media_type {
            self.inner
                .iter()
                .filter(|genre| genre.media_type().eq(&media_type))
                .collect()
        } else {
            self.inner.iter().collect()
        };

        let genres = if let Some(age_restriction) = age_restriction {
            genres
                .iter()
                .filter(|genre| genre.age_restriction().eq(&age_restriction))
                .map(Deref::deref)
                .collect()
        } else {
            genres
        };

        genres.into_iter().map(Clone::clone).collect()
    }

    /// Filters the genres by media type
    /// # Arguments
    /// * `media_type` - The media type to filter
    /// # Returns
    /// The filtered genres
    pub fn filter_media_type(&self, media_type: MediaType) -> Vec<Genre> {
        self.filter(Some(media_type), None)
    }

    /// Filters the genres by age restriction
    /// # Arguments
    /// * `age_restriction` - The age restriction to filter
    /// # Returns
    /// The filtered genres
    pub fn filter_age_restriction(&self, age_restriction: AgeRestriction) -> Vec<Genre> {
        self.filter(None, Some(age_restriction))
    }
}

impl Genres {
    /// Filters the genres by gif media type
    /// # Returns
    /// The filtered genres
    pub fn gifs(&self) -> Vec<Genre> {
        self.filter_media_type(MediaType::Gif)
    }

    /// Filters the genres by image media type
    /// # Returns
    /// The filtered genres
    pub fn images(&self) -> Vec<Genre> {
        self.filter_media_type(MediaType::Image)
    }

    /// Filters the genres by sfw age restriction
    /// # Returns
    /// The filtered genres
    pub fn sfw(&self) -> Vec<Genre> {
        self.filter_age_restriction(AgeRestriction::Sfw)
    }

    /// Filters the genres by nsfw age restriction
    /// # Returns
    /// The filtered genres
    pub fn nsfw(&self) -> Vec<Genre> {
        self.filter_age_restriction(AgeRestriction::Nsfw)
    }

    /// Filters the genres by sfw gif media type
    /// # Returns
    /// The filtered genres
    pub fn sfw_gifs(&self) -> Vec<Genre> {
        self.filter(Some(MediaType::Gif), Some(AgeRestriction::Sfw))
    }

    /// Filters the genres by nsfw gif media type
    /// # Returns
    /// The filtered genres
    pub fn nsfw_gifs(&self) -> Vec<Genre> {
        self.filter(Some(MediaType::Gif), Some(AgeRestriction::Nsfw))
    }

    /// Filters the genres by sfw image media type
    /// # Returns
    /// The filtered genres
    pub fn sfw_images(&self) -> Vec<Genre> {
        self.filter(Some(MediaType::Image), Some(AgeRestriction::Sfw))
    }

    /// Filters the genres by nsfw image media type
    /// # Returns
    /// The filtered genres
    pub fn nsfw_images(&self) -> Vec<Genre> {
        self.filter(Some(MediaType::Image), Some(AgeRestriction::Nsfw))
    }
}

impl Genres {
    /// Checks if the given genre is contains
    /// # Arguments
    /// * `genre` - The genre to check
    /// # Returns
    /// `true` if the genres contains the given genre, `false` otherwise
    pub fn contains(&self, genre: &Genre) -> bool {
        self.inner.contains(genre)
    }

    /// Filters the genres and checks if the given genre is contains
    /// # Arguments
    /// * `genre` - The genre to check
    /// * `media_type` - The media type to filter. If `None`, it will not filter by media type
    /// * `age_restriction` - The age restriction to filter. If `None`, it will not filter by age restriction
    /// # Returns
    /// `true` if the genres contains the given genre, `false` otherwise
    pub fn contains_filter(
        &self,
        genre: &Genre,
        media_type: Option<MediaType>,
        age_restriction: Option<AgeRestriction>,
    ) -> bool {
        let genres = self.filter(media_type, age_restriction);
        genres.contains(genre)
    }

    /// Filters the genres and checks if the given genre is contains
    /// # Arguments
    /// * `genre` - The genre to check
    /// * `media_type` - The media type to filter. If `None`, it will not filter by media type
    pub fn contains_filter_media_type(&self, genre: &Genre, media_type: MediaType) -> bool {
        self.contains_filter(genre, Some(media_type), None)
    }

    /// Filters the genres and checks if the given genre is contains
    /// # Arguments
    /// * `genre` - The genre to check
    /// * `age_restriction` - The age restriction to filter. If `None`, it will not filter by age restriction
    /// # Returns
    /// `true` if the genres contains the given genre, `false` otherwise
    pub fn contains_filter_age_restriction(
        &self,
        genre: &Genre,
        age_restriction: AgeRestriction,
    ) -> bool {
        self.contains_filter(genre, None, Some(age_restriction))
    }
}

impl Genres {
    /// Checks if the gif genres contains the given genre
    /// # Arguments
    /// * `genre` - The genre to check
    /// # Returns
    /// `true` if the gif genres contains the given genre, `false` otherwise
    pub fn contains_gifs(&self, genre: &Genre) -> bool {
        self.contains_filter_media_type(genre, MediaType::Gif)
    }

    /// Checks if the image genres contains the given genre
    /// # Arguments
    /// * `genre` - The genre to check
    /// # Returns
    /// `true` if the image genres contains the given genre, `false` otherwise
    pub fn contains_images(&self, genre: &Genre) -> bool {
        self.contains_filter_media_type(genre, MediaType::Image)
    }

    /// Checks if the sfw genres contains the given genre
    /// # Arguments
    /// * `genre` - The genre to check
    /// # Returns
    /// `true` if the sfw genres contains the given genre, `false` otherwise
    pub fn contains_sfw(&self, genre: &Genre) -> bool {
        self.contains_filter_age_restriction(genre, AgeRestriction::Sfw)
    }

    /// Checks if the nsfw genres contains the given genre
    /// # Arguments
    /// * `genre` - The genre to check
    /// # Returns
    /// `true` if the nsfw genres contains the given genre, `false` otherwise
    pub fn contains_nsfw(&self, genre: &Genre) -> bool {
        self.contains_filter_age_restriction(genre, AgeRestriction::Nsfw)
    }
}

impl Deref for Genres {
    type Target = Vec<Genre>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
