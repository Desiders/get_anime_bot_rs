use std::borrow::Cow;

use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUser {
    id: Uuid,
    tg_id: i64,
    language_code: Option<Cow<'static, str>>,
    show_nsfw: Option<bool>,
}

impl CreateUser {
    pub fn new(
        id: Uuid,
        tg_id: i64,
        language_code: Option<Cow<'static, str>>,
        show_nsfw: Option<bool>,
    ) -> Self {
        Self {
            id,
            tg_id,
            language_code: language_code.map(Into::into),
            show_nsfw,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn tg_id(&self) -> i64 {
        self.tg_id
    }

    pub fn language_code(&self) -> Option<&str> {
        self.language_code.as_deref()
    }

    pub fn show_nsfw(&self) -> Option<bool> {
        self.show_nsfw
    }
}
