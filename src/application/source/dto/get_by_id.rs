use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSourceById {
    id: Uuid,
}

impl GetSourceById {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
