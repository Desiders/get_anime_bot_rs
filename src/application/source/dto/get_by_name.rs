use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSourceByName {
    name: Cow<'static, str>,
}

impl GetSourceByName {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
