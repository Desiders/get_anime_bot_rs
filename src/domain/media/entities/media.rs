use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
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
