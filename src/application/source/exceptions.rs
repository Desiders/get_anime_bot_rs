use uuid::Uuid;

use crate::application::common::exceptions::ApplicationException;

use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
#[error("Source with name `{name}` and url `{url}` already exists: {message}")]
pub struct SourceNameAndUrlAlreadyExists {
    name: String,
    url: String,
    message: Cow<'static, str>,
}

impl SourceNameAndUrlAlreadyExists {
    pub fn new(name: String, url: String, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name,
            url,
            message: message.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Source with id `{id}` doesn't exist: {message}")]
pub struct SourceIdNotExist {
    id: Uuid,
    message: Cow<'static, str>,
}

impl SourceIdNotExist {
    pub fn new(id: Uuid, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id,
            message: message.into(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Source with name `{name}` and url `{url}` doesn't exist: {message}")]
pub struct SourceNameAndUrlNotExist {
    name: Cow<'static, str>,
    url: Cow<'static, str>,
    message: Cow<'static, str>,
}

impl SourceNameAndUrlNotExist {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        url: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            message: message.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl ApplicationException for SourceNameAndUrlAlreadyExists {}
impl ApplicationException for SourceIdNotExist {}
impl ApplicationException for SourceNameAndUrlNotExist {}
