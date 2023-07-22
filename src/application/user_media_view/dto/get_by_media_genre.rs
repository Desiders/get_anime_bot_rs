#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaGenre {
    genre: Option<String>,
}

impl GetUserMediaViewByMediaGenre {
    pub fn new(genre: Option<String>) -> Self {
        Self { genre }
    }

    pub fn genre(&self) -> Option<&str> {
        self.genre.as_deref()
    }
}
