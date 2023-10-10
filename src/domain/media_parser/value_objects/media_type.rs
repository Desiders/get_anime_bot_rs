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

impl MediaType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gif => "gif",
            Self::Image => "image",
            Self::Unknown => "unknown",
        }
    }
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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

#[cfg(test)]
mod tests {
    use super::MediaType;

    #[test]
    fn test_media_type() {
        assert!(MediaType::Gif.is_gif());
        assert!(MediaType::Image.is_image());
        assert!(MediaType::Unknown.is_unknown());
    }

    #[test]
    fn test_media_type_from_str() {
        assert_eq!(MediaType::try_from("gif").unwrap(), MediaType::Gif);
        assert_eq!(MediaType::try_from("image").unwrap(), MediaType::Image);
        assert_eq!(MediaType::try_from("unknown").unwrap(), MediaType::Unknown);
        assert_eq!(MediaType::try_from("").unwrap(), MediaType::Unknown);
        assert!(MediaType::try_from("test").is_err());
    }
}
