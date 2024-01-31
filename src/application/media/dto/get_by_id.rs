use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMediaById<'a> {
    id: &'a Uuid,
}

impl<'a> GetMediaById<'a> {
    pub const fn new(id: &'a Uuid) -> Self {
        Self { id }
    }

    pub const fn id(&self) -> &Uuid {
        self.id
    }
}
