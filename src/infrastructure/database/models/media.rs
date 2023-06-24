use sqlx::{
    types::{time::Date, Uuid},
    FromRow,
};

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Media {
    pub id: Uuid,
    pub url: String,
    pub genre: Option<String>,
    pub media_type: String,
    pub is_sfw: Option<bool>,
    pub source_id: Uuid,
    pub created: Date,
}
