use sqlx::{
    types::{time::OffsetDateTime, Uuid},
    FromRow,
};

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Source {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub created: OffsetDateTime,
}
