use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaGenre {
    genre: Option<Cow<'static, str>>,
}

impl GetUserMediaViewByMediaGenre {
    pub fn new(genre: Option<Cow<'static, str>>) -> Self {
        Self { genre }
    }

    pub fn genre(&self) -> Option<&str> {
        self.genre.as_deref()
    }
}
