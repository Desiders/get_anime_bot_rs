use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserMediaView {
    pub id: Uuid,
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub created: OffsetDateTime,
}
