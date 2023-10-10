use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSourceByNameAndUrl {
    name: Cow<'static, str>,
    url: Cow<'static, str>,
}

impl GetSourceByNameAndUrl {
    pub fn new(name: impl Into<Cow<'static, str>>, url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
