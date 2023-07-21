use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUserLanguageCode {
    id: Uuid,
    language_code: String,
}

impl UpdateUserLanguageCode {
    pub fn new(id: Uuid, language_code: String) -> Self {
        Self { id, language_code }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn language_code(&self) -> &str {
        &self.language_code
    }
}
