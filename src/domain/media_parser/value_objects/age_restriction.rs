use crate::domain::media_parser::exceptions::AgeRestrictionParse as AgeRestrictionParseError;

use serde::Deserialize;
use std::fmt::Display;

/// Age restriction of a media
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum AgeRestriction {
    Sfw,
    Nsfw,
    Unknown,
}

impl AgeRestriction {
    /// Returns `true` if the age restriction is [`AgeRestriction::Sfw`]
    pub const fn is_sfw(self) -> bool {
        matches!(self, Self::Sfw)
    }

    /// Returns `true` if the age restriction is [`AgeRestriction::Nsfw`]
    pub const fn is_nsfw(self) -> bool {
        matches!(self, Self::Nsfw)
    }

    /// Returns `true` if the age restriction is [`AgeRestriction::Unknown`]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl Display for AgeRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgeRestriction::Sfw => write!(f, "sfw"),
            AgeRestriction::Nsfw => write!(f, "nsfw"),
            AgeRestriction::Unknown => write!(f, "unknown"),
        }
    }
}

impl<'a> TryFrom<&'a str> for AgeRestriction {
    type Error = AgeRestrictionParseError<'a>;

    fn try_from(raw_age_restriction: &'a str) -> Result<Self, Self::Error> {
        match raw_age_restriction {
            "sfw" => Ok(Self::Sfw),
            "nsfw" => Ok(Self::Nsfw),
            "" | "unknown" => Ok(Self::Unknown),
            _ => Err(AgeRestrictionParseError::new(
                raw_age_restriction,
                "Unknown age restriction",
            )),
        }
    }
}
