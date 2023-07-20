use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum MediaType {
    Gif,
    Image,
    Unknown,
}

impl MediaType {
    /// Returns `true` if the media type is [`MediaType::Gif`]
    pub const fn is_gif(&self) -> bool {
        matches!(self, Self::Gif)
    }

    /// Returns `true` if the media type is [`MediaType::Image`]
    pub const fn is_image(&self) -> bool {
        matches!(self, Self::Image)
    }

    /// Returns `true` if the media type is [`MediaType::Unknown`]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
