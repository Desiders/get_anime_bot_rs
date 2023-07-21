#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserByTgId {
    tg_id: i64,
}

impl GetUserByTgId {
    pub fn new(tg_id: i64) -> Self {
        Self { tg_id }
    }

    pub fn tg_id(&self) -> i64 {
        self.tg_id
    }
}
