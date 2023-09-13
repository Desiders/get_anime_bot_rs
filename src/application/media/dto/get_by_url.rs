use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMediaByUrl {
    url: Cow<'static, str>,
}

impl GetMediaByUrl {
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self { url: url.into() }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
