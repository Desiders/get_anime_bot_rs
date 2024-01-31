#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaGenre<'a> {
    genre: Option<&'a str>,
}

impl<'a> GetUserMediaViewByMediaGenre<'a> {
    pub const fn new(genre: Option<&'a str>) -> Self {
        Self { genre }
    }

    pub const fn genre(&self) -> Option<&str> {
        self.genre
    }
}
