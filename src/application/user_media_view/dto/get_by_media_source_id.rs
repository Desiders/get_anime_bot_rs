use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaSourceId<'a> {
    source_id: &'a Uuid,
}

impl<'a> GetUserMediaViewByMediaSourceId<'a> {
    pub fn new(source_id: &'a Uuid) -> Self {
        Self { source_id }
    }

    pub fn source_id(&self) -> &Uuid {
        self.source_id
    }
}
