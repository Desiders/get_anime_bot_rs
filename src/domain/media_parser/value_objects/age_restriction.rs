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

#[cfg(test)]
mod tests {
    use super::AgeRestriction;

    #[test]
    fn test_age_restriction() {
        assert!(AgeRestriction::Sfw.is_sfw());
        assert!(!AgeRestriction::Sfw.is_nsfw());
        assert!(!AgeRestriction::Sfw.is_unknown());

        assert!(!AgeRestriction::Nsfw.is_sfw());
        assert!(AgeRestriction::Nsfw.is_nsfw());
        assert!(!AgeRestriction::Nsfw.is_unknown());

        assert!(!AgeRestriction::Unknown.is_sfw());
        assert!(!AgeRestriction::Unknown.is_nsfw());
        assert!(AgeRestriction::Unknown.is_unknown());
    }

    #[test]
    fn test_age_restriction_from_str() {
        assert_eq!(
            AgeRestriction::try_from("sfw").unwrap(),
            AgeRestriction::Sfw
        );
        assert_eq!(
            AgeRestriction::try_from("nsfw").unwrap(),
            AgeRestriction::Nsfw
        );
        assert_eq!(
            AgeRestriction::try_from("unknown").unwrap(),
            AgeRestriction::Unknown
        );
        assert_eq!(
            AgeRestriction::try_from("").unwrap(),
            AgeRestriction::Unknown
        );
    }
}
