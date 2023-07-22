use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaSourceId {
    source_id: Uuid,
}

impl GetUserMediaViewByMediaSourceId {
    pub fn new(source_id: Uuid) -> Self {
        Self { source_id }
    }

    pub fn source_id(&self) -> Uuid {
        self.source_id
    }
}
