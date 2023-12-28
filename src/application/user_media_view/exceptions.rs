use crate::application::common::exceptions::ApplicationException;

use std::borrow::Cow;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
#[error(
    "User media view with user id `{user_id}` and media id `{media_id}` already exists: {message}"
)]
pub struct UserMediaViewUserIdAndMediaIdAlreadyExists {
    user_id: Uuid,
    media_id: Uuid,
    message: Cow<'static, str>,
}

impl UserMediaViewUserIdAndMediaIdAlreadyExists {
    pub fn new(user_id: Uuid, media_id: Uuid, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            user_id,
            media_id,
            message: message.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("User media view with id `{id}` doesn't exist: {message}")]
pub struct UserMediaViewIdNotExist {
    id: Uuid,
    message: Cow<'static, str>,
}

impl UserMediaViewIdNotExist {
    pub fn new(id: Uuid, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id,
            message: message.into(),
        }
    }
}

impl ApplicationException for UserMediaViewUserIdAndMediaIdAlreadyExists {}
impl ApplicationException for UserMediaViewIdNotExist {}
