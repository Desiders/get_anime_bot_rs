use crate::domain::media_parser::{
    exceptions::MediaParse as MediaParseError,
    value_objects::{AgeRestriction, GenreName, MediaType},
};

use serde::Deserialize;
use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Genre {
    name: Cow<'static, GenreName>,
    media_type: MediaType,
    age_restriction: AgeRestriction,
}

impl Genre {
    /// Creates a new genre
    /// # Arguments
    /// * `name` - The name of the genre
    /// * `media_type` - The media type of the genre
    /// * `age_restriction` - The age restriction of the genre
    pub fn new(
        name: impl Into<Cow<'static, GenreName>>,
        media_type: MediaType,
        age_restriction: AgeRestriction,
    ) -> Self {
        Self {
            name: name.into(),
            media_type,
            age_restriction,
        }
    }

    /// Creates a new gif genre
    /// # Arguments
    /// * `name` - The name of the genre
    /// * `age_restriction` - The age restriction of the genre
    pub fn new_gif(
        name: impl Into<Cow<'static, GenreName>>,
        age_restriction: AgeRestriction,
    ) -> Self {
        Genre::new(name, MediaType::Gif, age_restriction)
    }

    /// Creates a new image genre
    /// # Arguments
    /// * `name` - The name of the genre
    /// * `age_restriction` - The age restriction of the genre
    pub fn new_image(
        name: impl Into<Cow<'static, GenreName>>,
        age_restriction: AgeRestriction,
    ) -> Self {
        Genre::new(name, MediaType::Image, age_restriction)
    }

    /// Creates a new sfw gif genre
    /// # Arguments
    /// * `name` - The name of the genre
    pub fn new_sfw_gif(name: impl Into<Cow<'static, GenreName>>) -> Self {
        Genre::new_gif(name, AgeRestriction::Sfw)
    }

    /// Creates a new sfw image genre
    /// # Arguments
    /// * `name` - The name of the genre
    pub fn new_sfw_image(name: impl Into<Cow<'static, GenreName>>) -> Self {
        Genre::new_image(name, AgeRestriction::Sfw)
    }

    /// Creates a new nsfw gif genre
    /// # Arguments
    /// * `name` - The name of the genre
    pub fn new_nsfw_gif(name: impl Into<Cow<'static, GenreName>>) -> Self {
        Genre::new_gif(name, AgeRestriction::Nsfw)
    }

    /// Creates a new nsfw image genre
    /// # Arguments
    /// * `name` - The name of the genre
    pub fn new_nsfw_image(name: impl Into<Cow<'static, GenreName>>) -> Self {
        Genre::new_image(name, AgeRestriction::Nsfw)
    }
}

/// Macro to create a vector of sfw gif genres
#[macro_export]
macro_rules! vec_new_sfw_gif {
    ($($name:expr),* $(,)?) => {
        vec![$(Genre::new_sfw_gif($name),)*]
    };
}

pub use vec_new_sfw_gif;

/// Macro to create a vector of sfw image genres
#[macro_export]
macro_rules! vec_new_sfw_image {
    ($($name:expr),* $(,)?) => {
        vec![$(Genre::new_sfw_image($name),)*]
    };
}

pub use vec_new_sfw_image;

/// Macro to create a vector of nsfw gif genres
#[macro_export]
macro_rules! vec_new_nsfw_gif {
    ($($name:expr),* $(,)?) => {
        vec![$(Genre::new_nsfw_gif($name),)*]
    };
}

pub use vec_new_nsfw_gif;

/// Macro to create a vector of nsfw image genres
#[macro_export]
macro_rules! vec_new_nsfw_image {
    ($($name:expr),* $(,)?) => {
        vec![$(Genre::new_nsfw_image($name),)*]
    };
}

pub use vec_new_nsfw_image;

impl Genre {
    /// Returns the name of the genre
    pub fn name(&self) -> &GenreName {
        &self.name
    }

    /// Returns the media type of the genre
    pub const fn media_type(&self) -> &MediaType {
        &self.media_type
    }

    /// Returns the age restriction of the genre
    pub const fn age_restriction(&self) -> &AgeRestriction {
        &self.age_restriction
    }
}

impl Genre {
    /// Returns `true` if the age restriction is [`AgeRestriction::Sfw`]
    pub const fn is_sfw(&self) -> bool {
        self.age_restriction.is_sfw()
    }

    /// Returns `true` if the age restriction is [`AgeRestriction::Nsfw`]
    pub const fn is_nsfw(&self) -> bool {
        self.age_restriction.is_nsfw()
    }

    /// Returns `true` if the age restriction is [`AgeRestriction::Unknown`]
    pub const fn age_restriction_is_unknown(&self) -> bool {
        self.age_restriction.is_unknown()
    }

    /// Returns `true` if the media type is [`MediaType::Gif`]
    pub const fn is_gif(&self) -> bool {
        self.media_type.is_gif()
    }

    /// Returns `true` if the media type is [`MediaType::Image`]
    pub const fn is_image(&self) -> bool {
        self.media_type.is_image()
    }

    /// Returns `true` if the media type is [`MediaType::Unknown`]
    pub const fn media_type_is_unknown(&self) -> bool {
        self.media_type.is_unknown()
    }
}

impl Display for Genre {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{name}_{media_type}_{age_restriction}",
            name = self.name(),
            media_type = self.media_type(),
            age_restriction = self.age_restriction()
        )
    }
}

impl<'a> TryFrom<&'a str> for Genre {
    type Error = MediaParseError<'a>;

    fn try_from(raw_genre: &'a str) -> Result<Self, Self::Error> {
        let mut parts = raw_genre.split('_');

        let Some(name) = parts.next() else {
            return Err(MediaParseError::NoNameProvided);
        };

        let media_type: MediaType = match parts.next() {
            Some(media_type) => media_type.try_into()?,
            None => return Err(MediaParseError::NoMediaTypeProvided),
        };

        let age_restriction: AgeRestriction = match parts.next() {
            Some(age_restriction) => age_restriction.try_into()?,
            None => return Err(MediaParseError::NoAgeRestrictionProvided),
        };

        Ok(Self::new(name.to_owned(), media_type, age_restriction))
    }
}

#[cfg(test)]
mod tests {
    use super::{AgeRestriction, Genre, MediaType};

    #[test]
    fn test_genre() {
        let genre = Genre::new("test", MediaType::Gif, AgeRestriction::Sfw);

        assert_eq!(genre.name(), "test");
        assert_eq!(genre.media_type(), &MediaType::Gif);
        assert_eq!(genre.age_restriction(), &AgeRestriction::Sfw);

        assert!(genre.is_sfw());
        assert!(!genre.is_nsfw());
        assert!(!genre.age_restriction_is_unknown());
        assert!(genre.is_gif());
        assert!(!genre.is_image());
        assert!(!genre.media_type_is_unknown());
    }

    #[test]
    fn test_genre_from_str() {
        let genre: Genre = "test_gif_sfw".try_into().unwrap();

        assert_eq!(genre.name(), "test");
        assert_eq!(genre.media_type(), &MediaType::Gif);
        assert_eq!(genre.age_restriction(), &AgeRestriction::Sfw);

        assert!(genre.is_sfw());
        assert!(!genre.is_nsfw());
        assert!(!genre.age_restriction_is_unknown());
        assert!(genre.is_gif());
        assert!(!genre.is_image());
        assert!(!genre.media_type_is_unknown());

        let genre: Genre = "test_image_nsfw".try_into().unwrap();

        assert_eq!(genre.name(), "test");
        assert_eq!(genre.media_type(), &MediaType::Image);
        assert_eq!(genre.age_restriction(), &AgeRestriction::Nsfw);

        assert!(!genre.is_sfw());
        assert!(genre.is_nsfw());
        assert!(!genre.age_restriction_is_unknown());
        assert!(!genre.is_gif());
        assert!(genre.is_image());
        assert!(!genre.media_type_is_unknown());

        let genre: Genre = "test_gif_unknown".try_into().unwrap();

        assert_eq!(genre.name(), "test");
        assert_eq!(genre.media_type(), &MediaType::Gif);
        assert_eq!(genre.age_restriction(), &AgeRestriction::Unknown);

        assert!(!genre.is_sfw());
        assert!(!genre.is_nsfw());
        assert!(genre.age_restriction_is_unknown());
        assert!(genre.is_gif());
        assert!(!genre.is_image());
        assert!(!genre.media_type_is_unknown());

        let genre: Genre = "test_image_unknown".try_into().unwrap();

        assert_eq!(genre.name(), "test");
        assert_eq!(genre.media_type(), &MediaType::Image);
        assert_eq!(genre.age_restriction(), &AgeRestriction::Unknown);

        assert!(!genre.is_sfw());
        assert!(!genre.is_nsfw());
        assert!(genre.age_restriction_is_unknown());
        assert!(!genre.is_gif());
        assert!(genre.is_image());
        assert!(!genre.media_type_is_unknown());
    }
}
