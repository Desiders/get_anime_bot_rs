use std::borrow::Cow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMediaByInfoUnviewedByUser {
    user_id: Uuid,
    genre: Option<Cow<'static, str>>,
    media_type: Cow<'static, str>,
    is_sfw: Option<bool>,
    offset: Option<u64>,
    limit: Option<u64>,
}

impl GetMediaByInfoUnviewedByUser {
    pub fn new(
        user_id: Uuid,
        genre: Option<Cow<'static, str>>,
        media_type: impl Into<Cow<'static, str>>,
        is_sfw: Option<bool>,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            user_id,
            genre,
            media_type: media_type.into(),
            is_sfw,
            offset,
            limit,
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
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
