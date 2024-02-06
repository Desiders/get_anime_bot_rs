use crate::domain::media::entities::MediaStats as MediaStatsEntity;

use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct MediaStats {
    pub total: i64,
    pub gif: i64,
    pub image: i64,
    pub unknown: i64,
    pub sfw: i64,
    pub nsfw: i64,
}

impl From<MediaStats> for MediaStatsEntity {
    fn from(media: MediaStats) -> Self {
        Self {
            total: media.total,
            gif: media.gif,
            image: media.image,
            unknown: media.unknown,
            sfw: media.sfw,
            nsfw: media.nsfw,
        }
    }
}
