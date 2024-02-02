use sqlx::{
    types::{time::OffsetDateTime, Uuid},
    FromRow,
};

use crate::domain::media::entities::Media as MediaEntity;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
#[allow(clippy::struct_field_names)]
pub struct Media {
    pub id: Uuid,
    pub url: String,
    pub genre: Option<String>,
    pub media_type: String,
    pub is_sfw: Option<bool>,
    pub source_id: Uuid,
    pub created: OffsetDateTime,
}

impl From<Media> for MediaEntity {
    fn from(media: Media) -> Self {
        Self {
            id: media.id,
            url: media.url,
            genre: media.genre,
            media_type: media.media_type,
            is_sfw: media.is_sfw,
            source_id: media.source_id,
            created: media.created,
        }
    }
}
