use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByUserId {
    user_id: Uuid,
}

impl GetUserMediaViewByUserId {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}
