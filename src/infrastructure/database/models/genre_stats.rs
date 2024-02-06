use crate::domain::media::entities::GenreStats as GenreStatsEntity;

use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct GenreStats {
    pub total: i64,
    pub genre: String,
    pub media_type: String,
    pub is_sfw: bool,
}

impl From<GenreStats> for GenreStatsEntity {
    fn from(genre: GenreStats) -> Self {
        Self {
            total: genre.total,
            genre: genre.genre,
            media_type: genre.media_type,
            is_sfw: genre.is_sfw,
        }
    }
}
