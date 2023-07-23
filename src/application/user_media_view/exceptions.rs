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

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn media_id(&self) -> Uuid {
        self.media_id
    }

    pub fn message(&self) -> &str {
        &self.message
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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl ApplicationException for UserMediaViewUserIdAndMediaIdAlreadyExists {}
impl ApplicationException for UserMediaViewIdNotExist {}
