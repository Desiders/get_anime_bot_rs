use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUserMediaView<'a> {
    id: &'a Uuid,
    user_id: &'a Uuid,
    media_id: &'a Uuid,
}

impl<'a> CreateUserMediaView<'a> {
    pub const fn new(id: &'a Uuid, user_id: &'a Uuid, media_id: &'a Uuid) -> Self {
        Self {
            id,
            user_id,
            media_id,
        }
    }

    pub const fn id(&self) -> &Uuid {
        self.id
    }

    pub const fn user_id(&self) -> &Uuid {
        self.user_id
    }

    pub const fn media_id(&self) -> &Uuid {
        self.media_id
    }
}
