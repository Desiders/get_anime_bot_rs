use async_trait::async_trait;

use crate::application::{
    common::exceptions::{BeginError, CommitError, RollbackError},
    media::traits::{MediaReader, MediaRepo},
    source::traits::{SourceReader, SourceRepo},
    user::traits::{UserReader, UserRepo},
    user_media_view::traits::{UserMediaViewReader, UserMediaViewRepo},
};

/// A factory for creating [`UnitOfWork`]s.
/// # Notes
/// This need to avoid sharing [`UnitOfWork`]s directly, because [`UnitOfWork`]s methods need `&mut self`
/// and it is not possible to share it without [`std::sync::Arc`] or [`std::sync::Mutex`].
///
/// If you have ideas how to do it better, please, create an issue or PR.
pub trait UnitOfWorkFactory {
    type UnitOfWork: UnitOfWork;

    fn new_unit_of_work(&self) -> Self::UnitOfWork;
}

#[async_trait]
pub trait UnitOfWork {
    type Connection<'a>
    where
        Self: 'a;

    /// Gets a connection of this transaction or savepoint.
    /// If there are no active transactions or savepoints, begins a new transaction or savepoint and returns a connection to it.
    async fn connection(&mut self) -> Result<Self::Connection<'_>, BeginError>;

    /// Begins a new transaction or savepoint and returns a connection to it
    async fn begin(&mut self) -> Result<(), BeginError>;

    /// Commits this transaction or savepoint, persisting any changes it has made
    async fn commit(&mut self) -> Result<(), CommitError>;

    /// Aborts this transaction or savepoint and rolls back any changes it has made
    async fn rollback(&mut self) -> Result<(), RollbackError>;

    /// Creates a new instance of [`UserRepo`] with a connection of this transaction or savepoint.
    /// If there are no active transactions or savepoints, begins a new transaction or savepoint and returns a connection to it.
    async fn user_repo(&mut self) -> Result<Box<dyn UserRepo + Send + '_>, BeginError>;

    /// Creates a new instance of [`UserReader`] with a connection of this transaction or savepoint.
    /// If there are no active transactions or savepoints, begins a new transaction or savepoint and returns a connection to it.
    async fn user_reader(&mut self) -> Result<Box<dyn UserReader + Send + '_>, BeginError>;

    /// Creates a new instance of [`SourceRepo`] with a connection of this transaction or savepoint.
    /// If there are no active transactions or savepoints, begins a new transaction or savepoint and returns a connection to it.
    async fn source_repo(&mut self) -> Result<Box<dyn SourceRepo + Send + '_>, BeginError>;

    /// Creates a new instance of [`SourceReader`] with a connection of this transaction or savepoint.
    /// If there are no active transactions or savepoints, begins a new transaction or savepoint and returns a connection to it.
    async fn source_reader(&mut self) -> Result<Box<dyn SourceReader + Send + '_>, BeginError>;

    /// Creates a new instance of [`MediaRepo`] with a connection of this transaction or savepoint.
    /// If there are no active transactions or savepoints, begins a new transaction or savepoint and returns a connection to it.
    async fn media_repo(&mut self) -> Result<Box<dyn MediaRepo + Send + '_>, BeginError>;

    /// Creates a new instance of [`MediaReader`] with a connection of this transaction or savepoint.
    /// If there are no active transactions or savepoints, begins a new transaction or savepoint and returns a connection to it.
    async fn media_reader(&mut self) -> Result<Box<dyn MediaReader + Send + '_>, BeginError>;

    /// Creates a new instance of [`UserMediaViewRepo`] with a connection of this transaction or savepoint.
    /// If there are no active transactions or savepoints, begins a new transaction or savepoint and returns a connection to it.
    async fn user_media_view_repo(
        &mut self,
    ) -> Result<Box<dyn UserMediaViewRepo + Send + '_>, BeginError>;

    /// Creates a new instance of [`UserMediaViewReader`] with a connection of this transaction or savepoint.
    /// If there are no active transactions or savepoints, begins a new transaction or savepoint and returns a connection to it.
    async fn user_media_view_reader(
        &mut self,
    ) -> Result<Box<dyn UserMediaViewReader + Send + '_>, BeginError>;
}
