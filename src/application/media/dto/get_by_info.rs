#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMediaByInfo<'a> {
    genre: Option<&'a str>,
    media_type: &'a str,
    is_sfw: Option<bool>,
    offset: Option<u64>,
    limit: Option<u64>,
}

impl<'a> GetMediaByInfo<'a> {
    pub fn new(
        genre: Option<&'a str>,
        media_type: impl Into<&'a str>,
        is_sfw: Option<bool>,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            genre,
            media_type: media_type.into(),
            is_sfw,
            offset,
            limit,
        }
    }

    pub fn genre(&self) -> Option<&str> {
        self.genre
    }

    pub fn media_type(&self) -> &str {
        self.media_type
    }

    pub fn is_sfw(&self) -> Option<bool> {
        self.is_sfw
    }

    pub fn offset(&self) -> Option<u64> {
        self.offset
    }

    pub fn limit(&self) -> Option<u64> {
        self.limit
    }
}
