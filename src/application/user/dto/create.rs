use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUser {
    pub id: Uuid,
    pub tg_id: i64,
    pub language_code: Option<String>,
    pub show_nsfw: Option<bool>,
}
