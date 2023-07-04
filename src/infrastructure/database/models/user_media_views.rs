use sea_query::Iden;
use sqlx::{
    types::{time::Date, Uuid},
    FromRow,
};

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct UserMediaViews {
    pub id: Uuid,
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub created: Date,
}

#[derive(Iden, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserMediaViewsTable {
    Table,
    Id,
    UserId,
    MediaId,
    Created,
}
