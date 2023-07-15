use crate::application::common::traits::UnitOfWork;

use async_trait::async_trait;
use sqlx::{Connection, Database, Transaction};

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
    type BeginError = sqlx::Error;
    type CommitError = sqlx::Error;
    type RollbackError = sqlx::Error;

    type Connection<'a> = &'a mut DB::Connection where Self: 'a;

    fn connection(&mut self) -> Self::Connection<'_> {
        &mut self.conn
    }

    async fn begin(&'static mut self) -> Result<Self::Connection<'_>, Self::BeginError> {
        self.transaction = Some(self.conn.begin().await?);

        Ok(self.transaction.as_mut().unwrap())
    }

    async fn commit(&mut self) -> Result<(), Self::CommitError> {
        if let Some(transaction) = self.transaction.take() {
            transaction.commit().await
        } else {
            Ok(())
        }
    }

    async fn rollback(&mut self) -> Result<(), Self::RollbackError> {
        if let Some(transaction) = self.transaction.take() {
            transaction.rollback().await
        } else {
            Ok(())
        }
    }
}
