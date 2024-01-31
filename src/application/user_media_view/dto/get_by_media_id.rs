use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaId<'a> {
    media_id: &'a Uuid,
}

impl<'a> GetUserMediaViewByMediaId<'a> {
    pub const fn new(media_id: &'a Uuid) -> Self {
        Self { media_id }
    }

    pub const fn media_id(&self) -> &Uuid {
        self.media_id
    }
}
