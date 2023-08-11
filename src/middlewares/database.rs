use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use telers::{
    errors::EventErrorKind,
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
    router::Request,
};
use tokio::sync::Mutex;
use tracing::instrument;

use crate::infrastructure::database::SqlxUnitOfWork;

#[allow(clippy::module_name_repetitions)]
pub struct Database<DB>
where
    DB: sqlx::Database,
{
    pool: Pool<DB>,
}

impl<DB> Database<DB>
where
    DB: sqlx::Database,
{
    pub fn new(pool: Pool<DB>) -> Self {
        Self { pool }
    }
}

impl<DB> Database<DB>
where
    DB: sqlx::Database,
{
    /// Shutdown the connection pool
    pub async fn close(self) {
        self.pool.close().await;
    }
}

impl Clone for Database<Postgres> {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

#[async_trait]
impl<DB, Client> Middleware<Client> for Database<DB>
where
    DB: sqlx::Database,
    Client: Send + Sync + 'static,
{
    #[instrument(skip(self, request))]
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        let uow = Arc::new(Mutex::new(SqlxUnitOfWork::new(self.pool.clone())));

        request.context.insert("uow", Box::new(uow));

        Ok((request, EventReturn::Finish))
    }
}
