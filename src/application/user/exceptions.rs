use crate::application::common::exceptions::ApplicationException;

use std::borrow::Cow;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
#[error("User with tg id `{tg_id}` already exists: {message}")]
pub struct UserTgIdAlreadyExists {
    tg_id: i64,
    message: Cow<'static, str>,
}

impl UserTgIdAlreadyExists {
    pub fn new(tg_id: i64, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            tg_id,
            message: message.into(),
        }
    }

    pub fn tg_id(&self) -> i64 {
        self.tg_id
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, thiserror::Error)]
#[error("User with id `{id}` doesn't exist: {message}")]
pub struct UserIdNotExist {
    id: Uuid,
    message: Cow<'static, str>,
}

impl UserIdNotExist {
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
#[error("User with tg id `{tg_id}` doesn't exist: {message}")]
pub struct UserTgIdNotExist {
    tg_id: i64,
    message: Cow<'static, str>,
}

impl UserTgIdNotExist {
    pub fn new(tg_id: i64, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            tg_id,
            message: message.into(),
        }
    }

    pub fn tg_id(&self) -> i64 {
        self.tg_id
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl ApplicationException for UserTgIdAlreadyExists {}
impl ApplicationException for UserIdNotExist {}
impl ApplicationException for UserTgIdNotExist {}
