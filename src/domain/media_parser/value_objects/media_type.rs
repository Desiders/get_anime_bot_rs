use crate::domain::media_parser::exceptions::MediaTypeParse as MediaTypeParseError;

use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum MediaType {
    Gif,
    Image,
    Unknown,
}

impl MediaType {
    /// Returns `true` if the media type is [`MediaType::Gif`]
    pub const fn is_gif(self) -> bool {
        matches!(self, Self::Gif)
    }

    /// Returns `true` if the media type is [`MediaType::Image`]
    pub const fn is_image(self) -> bool {
        matches!(self, Self::Image)
    }

    /// Returns `true` if the media type is [`MediaType::Unknown`]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaType::Gif => write!(f, "gif"),
            MediaType::Image => write!(f, "image"),
            MediaType::Unknown => write!(f, "unknown"),
        }
    }
}

impl<'a> TryFrom<&'a str> for MediaType {
    type Error = MediaTypeParseError<'a>;

    fn try_from(raw_media_type: &'a str) -> Result<Self, Self::Error> {
        match raw_media_type {
            "gif" => Ok(Self::Gif),
            "image" => Ok(Self::Image),
            "" | "unknown" => Ok(Self::Unknown),
            _ => Err(MediaTypeParseError::new(
                raw_media_type,
                "Unknown media type",
            )),
        }
    }
}
