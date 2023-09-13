use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaType {
    media_type: Cow<'static, str>,
}

impl GetUserMediaViewByMediaType {
    pub fn new(media_type: impl Into<Cow<'static, str>>) -> Self {
        Self {
            media_type: media_type.into(),
        }
    }

    pub fn media_type(&self) -> &str {
        &self.media_type
    }
}
