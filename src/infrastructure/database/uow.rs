use async_trait::async_trait;
use sqlx::{Database, Transaction};

pub use crate::application::common::traits::UnitOfWork;

pub struct SqlxUnitOfWork<'a, DB: Database> {
    transaction: Transaction<'a, DB>,
}

#[async_trait]
impl<'a, DB> UnitOfWork for SqlxUnitOfWork<'a, DB>
where
    DB: Database,
{
    type CommitError = sqlx::Error;
    type RollbackError = sqlx::Error;

    async fn commit(self) -> Result<(), Self::CommitError> {
        self.transaction.commit().await
    }

    async fn rollback(self) -> Result<(), Self::RollbackError> {
        self.transaction.rollback().await
    }
}
