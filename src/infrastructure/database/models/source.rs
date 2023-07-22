use sqlx::{
    types::{time::OffsetDateTime, Uuid},
    FromRow,
};

use crate::domain::source::entities::Source as SourceEntity;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Source {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub created: OffsetDateTime,
}

impl From<Source> for SourceEntity {
    fn from(source: Source) -> Self {
        Self {
            id: source.id,
            name: source.name,
            url: source.url,
            created: source.created,
        }
    }
}
