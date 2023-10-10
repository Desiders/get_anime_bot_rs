use crate::infrastructure::database::SqlxUnitOfWorkFactory;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use telers::{
    errors::EventErrorKind,
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
    router::Request,
};
use tracing::instrument;

#[allow(clippy::module_name_repetitions)]
pub struct Database<DB>
where
    DB: sqlx::Database,
{
    pool: Pool<DB>,
    uow_factory: Arc<SqlxUnitOfWorkFactory<DB>>,
}

impl<DB> Database<DB>
where
    DB: sqlx::Database,
{
    pub fn new(pool: Pool<DB>) -> Self {
        Self {
            pool: pool.clone(),
            uow_factory: Arc::new(SqlxUnitOfWorkFactory::new(pool)),
        }
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
            uow_factory: self.uow_factory.clone(),
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
        request
            .context
            .insert("uow_factory", Box::new(self.uow_factory.clone()));

        Ok((request, EventReturn::Finish))
    }
}
