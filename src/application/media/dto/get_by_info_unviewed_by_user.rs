use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMediaByInfoUnviewedByUser<'a> {
    user_id: &'a Uuid,
    genre: Option<&'a str>,
    media_type: &'a str,
    is_sfw: Option<bool>,
    offset: Option<u64>,
    limit: Option<u64>,
}

impl<'a> GetMediaByInfoUnviewedByUser<'a> {
    pub const fn new(
        user_id: &'a Uuid,
        genre: Option<&'a str>,
        media_type: &'a str,
        is_sfw: Option<bool>,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            user_id,
            genre,
            media_type,
            is_sfw,
            offset,
            limit,
        }
    }

    pub const fn user_id(&self) -> &Uuid {
        self.user_id
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
