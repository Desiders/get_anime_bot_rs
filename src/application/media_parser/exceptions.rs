use crate::{
    application::common::exceptions::{ApplicationException, UnexpectedError},
    domain::media_parser::entities::Genre,
};

use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
#[error("Get media with genre `{genre:?}` failed: {message}")]
pub struct MediaGetException {
    genre: Genre,
    message: Cow<'static, str>,
}

impl MediaGetException {
    pub fn new(genre: Genre, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            genre,
            message: message.into(),
        }
    }
}

impl ApplicationException for MediaGetException {}
impl UnexpectedError for MediaGetException {}
