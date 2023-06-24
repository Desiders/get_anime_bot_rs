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
