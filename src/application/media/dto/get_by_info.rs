#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMediaByInfo<'a> {
    genre: Option<&'a str>,
    media_type: &'a str,
    is_sfw: Option<bool>,
    offset: Option<u64>,
    limit: Option<u64>,
}

impl<'a> GetMediaByInfo<'a> {
    pub const fn new(
        genre: Option<&'a str>,
        media_type: &'a str,
        is_sfw: Option<bool>,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            genre,
            media_type,
            is_sfw,
            offset,
            limit,
        }
    }

    pub const fn genre(&self) -> Option<&str> {
        self.genre
    }

    pub const fn media_type(&self) -> &str {
        self.media_type
    }

    pub const fn is_sfw(&self) -> Option<bool> {
        self.is_sfw
    }

    pub const fn offset(&self) -> Option<u64> {
        self.offset
    }

    pub const fn limit(&self) -> Option<u64> {
        self.limit
    }
}
