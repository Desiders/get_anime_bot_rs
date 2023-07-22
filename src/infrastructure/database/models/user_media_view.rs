use crate::domain::user_media_view::entities::UserMediaView as UserMediaViewEntity;
use sqlx::{
    types::{time::OffsetDateTime, Uuid},
    FromRow,
};

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct UserMediaView {
    pub id: Uuid,
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub created: OffsetDateTime,
}

impl From<UserMediaView> for UserMediaViewEntity {
    fn from(user_media_view: UserMediaView) -> Self {
        Self {
            id: user_media_view.id,
            user_id: user_media_view.user_id,
            media_id: user_media_view.media_id,
            created: user_media_view.created,
        }
    }
}
