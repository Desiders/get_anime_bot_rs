use std::borrow::Cow;
use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSource {
    id: Uuid,
    name: Cow<'static, str>,
    url: Cow<'static, str>,
}

impl CreateSource {
    pub fn new(
        id: Uuid,
        name: impl Into<Cow<'static, str>>,
        url: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            url: url.into(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
