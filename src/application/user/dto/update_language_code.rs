use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUserLanguageCode {
    pub id: Uuid,
    pub language_code: String,
}
