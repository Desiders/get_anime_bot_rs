use telers::extractors::FromContext;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, FromContext)]
#[context(key = "db_user")]
pub struct User {
    pub id: Uuid,
    pub tg_id: i64,
    pub language_code: Option<String>,
    pub show_nsfw: Option<bool>,
    pub created: OffsetDateTime,
}
