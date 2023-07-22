use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUserMediaView {
    id: Uuid,
    user_id: Uuid,
    media_id: Uuid,
}

impl CreateUserMediaView {
    pub fn new(id: Uuid, user_id: Uuid, media_id: Uuid) -> Self {
        Self {
            id,
            user_id,
            media_id,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn media_id(&self) -> Uuid {
        self.media_id
    }
}
