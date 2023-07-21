use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateMedia {
    id: Uuid,
    url: String,
    genre: Option<String>,
    media_type: String,
    is_sfw: Option<bool>,
    source_id: Uuid,
}

impl CreateMedia {
    pub fn new(
        id: Uuid,
        url: String,
        genre: Option<String>,
        media_type: String,
        is_sfw: Option<bool>,
        source_id: Uuid,
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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn url(&self) -> &str {
        &self.url
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

    pub fn source_id(&self) -> Uuid {
        self.source_id
    }
}
