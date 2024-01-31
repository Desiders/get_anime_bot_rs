use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateMedia<'a> {
    id: &'a Uuid,
    url: &'a str,
    genre: Option<&'a str>,
    media_type: &'a str,
    is_sfw: Option<bool>,
    source_id: &'a Uuid,
}

impl<'a> CreateMedia<'a> {
    pub const fn new(
        id: &'a Uuid,
        url: &'a str,
        genre: Option<&'a str>,
        media_type: &'a str,
        is_sfw: Option<bool>,
        source_id: &'a Uuid,
    ) -> Self {
        Self {
            id,
            url,
            genre,
            media_type,
            is_sfw,
            source_id,
        }
    }

    pub const fn id(&self) -> &Uuid {
        self.id
    }

    pub const fn url(&self) -> &str {
        self.url
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

    pub const fn source_id(&self) -> &Uuid {
        self.source_id
    }
}
