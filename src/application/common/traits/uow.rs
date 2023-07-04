use async_trait::async_trait;

#[async_trait]
pub trait UnitOfWork {
    type CommitError;
    type RollbackError;

    /// Commits this transaction or savepoint, persisting any changes it has made.
    async fn commit(self) -> Result<(), Self::CommitError>;

    /// Aborts this transaction or savepoint and rolls back any changes it has made.
    async fn rollback(self) -> Result<(), Self::RollbackError>;
}
