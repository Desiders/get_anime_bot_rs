use async_trait::async_trait;

use crate::application::{
    common::exceptions::{BeginError, CommitError, RollbackError},
    media::traits::{MediaReader, MediaRepo},
    source::traits::{SourceReader, SourceRepo},
    user::traits::{UserReader, UserRepo},
    user_media_view::traits::{UserMediaViewReader, UserMediaViewRepo},
};

#[async_trait]
pub trait UnitOfWork {
    type Connection<'a>
    where
        Self: 'a;

    // Transaction and savepoint management (begin)

    /// Gets a connection of this transaction or savepoint
    fn connection(&mut self) -> Self::Connection<'_>;

    /// Begins a new transaction or savepoint and returns a connection to it
    async fn begin(&'static mut self) -> Result<Self::Connection<'_>, BeginError>;

    /// Commits this transaction or savepoint, persisting any changes it has made
    async fn commit(&mut self) -> Result<(), CommitError>;

    /// Aborts this transaction or savepoint and rolls back any changes it has made
    async fn rollback(&mut self) -> Result<(), RollbackError>;

    // Transaction and savepoint management (end)
    // Repositories and readers for each entity (begin)

    /// Returns a new instance of [`UserRepo`]
    fn user_repo(&mut self) -> Box<dyn UserRepo + Send + '_>;
    /// Returns a new instance of [`UserReader`]
    fn user_reader(&mut self) -> Box<dyn UserReader + Send + '_>;

    /// Returns a new instance of [`SourceRepo`]
    fn source_repo(&mut self) -> Box<dyn SourceRepo + Send + '_>;
    /// Returns a new instance of [`SourceReader`]
    fn source_reader(&mut self) -> Box<dyn SourceReader + Send + '_>;

    /// Returns a new instance of [`MediaRepo`]
    fn media_repo(&mut self) -> Box<dyn MediaRepo + Send + '_>;
    /// Returns a new instance of [`MediaReader`]
    fn media_reader(&mut self) -> Box<dyn MediaReader + Send + '_>;

    /// Returns a new instance of [`UserMediaViewRepo`]
    fn user_media_view_repo(&mut self) -> Box<dyn UserMediaViewRepo + Send + '_>;
    /// Returns a new instance of [`UserMediaViewReader`]
    fn user_media_view_reader(&mut self) -> Box<dyn UserMediaViewReader + Send + '_>;

    // Repositories and readers for each entity (end)
}
