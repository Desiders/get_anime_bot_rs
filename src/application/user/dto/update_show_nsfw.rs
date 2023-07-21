use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUserShowNsfw {
    id: Uuid,
    show_nsfw: bool,
}

impl UpdateUserShowNsfw {
    pub fn new(id: Uuid, show_nsfw: bool) -> Self {
        Self { id, show_nsfw }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn show_nsfw(&self) -> bool {
        self.show_nsfw
    }
}
