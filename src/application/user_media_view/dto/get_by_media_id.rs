use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaId {
    media_id: Uuid,
}

impl GetUserMediaViewByMediaId {
    pub fn new(media_id: Uuid) -> Self {
        Self { media_id }
    }

    pub fn media_id(&self) -> Uuid {
        self.media_id
    }
}
