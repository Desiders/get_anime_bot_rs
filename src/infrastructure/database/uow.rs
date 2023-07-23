use crate::application::common::{
    exceptions::{BeginError, CommitError, RollbackError},
    traits::UnitOfWork,
};

use async_trait::async_trait;
use sqlx::{Connection, Database, Transaction};

impl From<sqlx::Error> for BeginError {
    fn from(error: sqlx::Error) -> Self {
        Self::new(error.to_string())
    }
}

impl From<sqlx::Error> for CommitError {
    fn from(error: sqlx::Error) -> Self {
        Self::new(error.to_string())
    }
}

impl From<sqlx::Error> for RollbackError {
    fn from(error: sqlx::Error) -> Self {
        Self::new(error.to_string())
    }
}

pub struct SqlxUnitOfWork<DB>
where
    DB: Database,
{
    conn: DB::Connection,
    transaction: Option<Transaction<'static, DB>>,
}

impl<DB> SqlxUnitOfWork<DB>
where
    DB: Database,
{
    pub fn new(conn: DB::Connection) -> Self {
        Self {
            conn,
            transaction: None,
        }
    }
}

#[async_trait]
impl<DB> UnitOfWork for SqlxUnitOfWork<DB>
where
    DB: Database,
{
    type Connection<'a> = &'a mut DB::Connection where Self: 'a;

    fn connection(&mut self) -> Self::Connection<'_> {
        &mut self.conn
    }

    async fn begin(&'static mut self) -> Result<Self::Connection<'_>, BeginError> {
        self.transaction = Some(self.conn.begin().await?);

        Ok(self.transaction.as_mut().unwrap())
    }

    async fn commit(&mut self) -> Result<(), CommitError> {
        if let Some(transaction) = self.transaction.take() {
            transaction.commit().await.map_err(Into::into)
        } else {
            Ok(())
        }
    }

    async fn rollback(&mut self) -> Result<(), RollbackError> {
        if let Some(transaction) = self.transaction.take() {
            transaction.rollback().await.map_err(Into::into)
        } else {
            Ok(())
        }
    }
}
