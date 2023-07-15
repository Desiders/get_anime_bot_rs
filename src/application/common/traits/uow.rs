use async_trait::async_trait;

#[async_trait]
pub trait UnitOfWork {
    type BeginError;
    type CommitError;
    type RollbackError;

    type Connection<'a>
    where
        Self: 'a;

    /// Gets a connection of this transaction or savepoint
    fn connection(&mut self) -> Self::Connection<'_>;

    /// Begins a new transaction or savepoint and returns a connection to it
    async fn begin(&'static mut self) -> Result<Self::Connection<'_>, Self::BeginError>;

    /// Commits this transaction or savepoint, persisting any changes it has made
    async fn commit(&mut self) -> Result<(), Self::CommitError>;

    /// Aborts this transaction or savepoint and rolls back any changes it has made
    async fn rollback(&mut self) -> Result<(), Self::RollbackError>;
}
