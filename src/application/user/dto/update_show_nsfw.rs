use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUserShowNsfw<'a> {
    id: &'a Uuid,
    show_nsfw: bool,
}

impl<'a> UpdateUserShowNsfw<'a> {
    pub const fn new(id: &'a Uuid, show_nsfw: bool) -> Self {
        Self { id, show_nsfw }
    }

    pub const fn id(&self) -> &Uuid {
        self.id
    }

    pub const fn show_nsfw(&self) -> bool {
        self.show_nsfw
    }
}
