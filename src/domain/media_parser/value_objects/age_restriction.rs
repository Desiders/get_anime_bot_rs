use serde::Deserialize;

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
