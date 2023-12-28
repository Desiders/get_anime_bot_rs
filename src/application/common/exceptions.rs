use crate::domain::common::exceptions::AppException;

use std::borrow::Cow;

pub trait ApplicationException: AppException {}

pub trait UnexpectedError: ApplicationException {}

#[derive(Debug, thiserror::Error)]
#[error("Repo error: {message}")]
pub struct RepoError {
    message: Cow<'static, str>,
}

impl RepoError {
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl ApplicationException for RepoError {}
impl UnexpectedError for RepoError {}

#[derive(Debug, thiserror::Error)]
pub enum RepoKind<RepoException>
where
    RepoException: ApplicationException,
{
    #[error(transparent)]
    Exception(RepoException),
    #[error(transparent)]
    Unexpected(RepoError),
}

impl<RepoException> RepoKind<RepoException>
where
    RepoException: ApplicationException,
{
    pub fn exception(exception: impl Into<RepoException>) -> Self {
        Self::Exception(exception.into())
    }

    pub fn unexpected(error: impl Into<RepoError>) -> Self {
        Self::Unexpected(error.into())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Begin error: {message}")]
pub struct BeginError {
    message: Cow<'static, str>,
}

impl BeginError {
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl ApplicationException for BeginError {}
impl UnexpectedError for BeginError {}

#[derive(Debug, thiserror::Error)]
#[error("Commit error: {message}")]
pub struct CommitError {
    message: Cow<'static, str>,
}

impl CommitError {
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl ApplicationException for CommitError {}
impl UnexpectedError for CommitError {}

#[derive(Debug, thiserror::Error)]
#[error("Rollback error: {message}")]
pub struct RollbackError {
    message: Cow<'static, str>,
}

impl RollbackError {
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl ApplicationException for RollbackError {}
impl UnexpectedError for RollbackError {}
