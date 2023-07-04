use time::Date;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub tg_id: i64,
    pub language_code: Option<String>,
    pub show_nsfw: Option<bool>,
    pub created: Date,
}
