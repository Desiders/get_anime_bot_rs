use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUserShowNsfw {
    pub id: Uuid,
    pub show_nsfw: bool,
}
