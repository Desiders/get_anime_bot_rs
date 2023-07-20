use sqlx::{
    types::{time::OffsetDateTime, Uuid},
    FromRow,
};

use crate::domain::user::entities::User as UserEntity;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: Uuid,
    pub tg_id: i64,
    pub language_code: Option<String>,
    pub show_nsfw: Option<bool>,
    pub created: OffsetDateTime,
}

impl From<User> for UserEntity {
    fn from(user: User) -> Self {
        UserEntity {
            id: user.id,
            tg_id: user.tg_id,
            language_code: user.language_code,
            show_nsfw: user.show_nsfw,
            created: user.created,
        }
    }
}
