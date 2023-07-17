use sqlx::{
    types::{time::Date, Uuid},
    FromRow,
};

use crate::application::user::dto;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: Uuid,
    pub tg_id: i64,
    pub language_code: Option<String>,
    pub show_nsfw: Option<bool>,
    pub created: Date,
}

impl From<User> for dto::User {
    fn from(user: User) -> Self {
        dto::User {
            id: user.id,
            tg_id: user.tg_id,
            language_code: user.language_code,
            show_nsfw: user.show_nsfw,
            created: user.created,
        }
    }
}
