#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaType {
    media_type: String,
}

impl GetUserMediaViewByMediaType {
    pub fn new(media_type: String) -> Self {
        Self { media_type }
    }

    pub fn media_type(&self) -> &str {
        &self.media_type
    }
}
