use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUserLanguageCode<'a> {
    id: &'a Uuid,
    language_code: &'a str,
}

impl<'a> UpdateUserLanguageCode<'a> {
    pub const fn new(id: &'a Uuid, language_code: &'a str) -> Self {
        Self { id, language_code }
    }

    pub const fn id(&self) -> &Uuid {
        self.id
    }

    pub const fn language_code(&self) -> &str {
        self.language_code
    }
}
