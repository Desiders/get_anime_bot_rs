#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaType<'a> {
    media_type: &'a str,
}

impl<'a> GetUserMediaViewByMediaType<'a> {
    pub const fn new(media_type: &'a str) -> Self {
        Self { media_type }
    }

    pub const fn media_type(&self) -> &str {
        self.media_type
    }
}
