use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUser<'a> {
    id: &'a Uuid,
    tg_id: i64,
    language_code: Option<&'a str>,
    show_nsfw: Option<bool>,
}

impl<'a> CreateUser<'a> {
    pub const fn new(
        id: &'a Uuid,
        tg_id: i64,
        language_code: Option<&'a str>,
        show_nsfw: Option<bool>,
    ) -> Self {
        Self {
            id,
            tg_id,
            language_code,
            show_nsfw,
        }
    }

    pub const fn id(&self) -> &Uuid {
        self.id
    }

    pub const fn tg_id(&self) -> i64 {
        self.tg_id
    }

    pub const fn language_code(&self) -> Option<&str> {
        self.language_code
    }

    pub const fn show_nsfw(&self) -> Option<bool> {
        self.show_nsfw
    }
}
