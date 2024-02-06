use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenreStats {
    pub total: i64,
    pub genre: String,
    pub media_type: String,
    pub is_sfw: bool,
}

impl Display for GenreStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "/{genre}_{media_type}_{is_sfw}: {total}",
            genre = self.genre,
            media_type = self.media_type,
            is_sfw = if self.is_sfw { "sfw" } else { "nsfw" },
            total = self.total
        )
    }
}
