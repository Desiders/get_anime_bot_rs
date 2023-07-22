use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub created: OffsetDateTime,
}
