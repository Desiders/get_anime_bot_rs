#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByUserTgId {
    user_tg_id: i64,
}

impl GetUserMediaViewByUserTgId {
    pub fn new(user_tg_id: i64) -> Self {
        Self { user_tg_id }
    }

    pub fn user_tg_id(&self) -> i64 {
        self.user_tg_id
    }
}
