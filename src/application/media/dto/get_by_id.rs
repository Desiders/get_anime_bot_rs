use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMediaById {
    id: Uuid,
}

impl GetMediaById {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
