use uuid::Uuid;

use crate::application::common::exceptions::ApplicationException;

use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
#[error("Media with url `{url}` and genre `{genre:?}` already exists: {message}")]
pub struct MediaUrlAndGenreAlreadyExists {
    url: String,
    genre: Option<String>,
    message: Cow<'static, str>,
}

impl MediaUrlAndGenreAlreadyExists {
    pub fn new(url: String, genre: Option<String>, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            url,
            genre,
            message: message.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Media with id `{id}` doesn't exist: {message}")]
pub struct MediaIdNotExist {
    id: Uuid,
    message: Cow<'static, str>,
}

impl MediaIdNotExist {
    pub fn new(id: Uuid, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id,
            message: message.into(),
        }
    }
}

impl ApplicationException for MediaUrlAndGenreAlreadyExists {}
impl ApplicationException for MediaIdNotExist {}
