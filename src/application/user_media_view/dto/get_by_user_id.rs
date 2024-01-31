use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByUserId<'a> {
    user_id: &'a Uuid,
}

impl<'a> GetUserMediaViewByUserId<'a> {
    pub const fn new(user_id: &'a Uuid) -> Self {
        Self { user_id }
    }

    pub const fn user_id(&self) -> &Uuid {
        self.user_id
    }
}
