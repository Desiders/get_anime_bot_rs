use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewById<'a> {
    id: &'a Uuid,
}

impl<'a> GetUserMediaViewById<'a> {
    pub const fn new(id: &'a Uuid) -> Self {
        Self { id }
    }

    pub const fn id(&self) -> &Uuid {
        self.id
    }
}
