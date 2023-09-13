use std::borrow::Cow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUserLanguageCode {
    id: Uuid,
    language_code: Cow<'static, str>,
}

impl UpdateUserLanguageCode {
    pub fn new(id: Uuid, language_code: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id,
            language_code: language_code.into(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn language_code(&self) -> &str {
        &self.language_code
    }
}
