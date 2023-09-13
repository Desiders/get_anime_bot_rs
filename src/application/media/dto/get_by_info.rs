use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMediaByInfo {
    genre: Option<Cow<'static, str>>,
    media_type: Cow<'static, str>,
    is_sfw: Option<bool>,
    offset: Option<u64>,
    limit: Option<u64>,
}

impl GetMediaByInfo {
    pub fn new(
        genre: Option<Cow<'static, str>>,
        media_type: impl Into<Cow<'static, str>>,
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
        self.genre.as_deref()
    }

    pub fn media_type(&self) -> &str {
        &self.media_type
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
