use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use telers::{
    errors::{EventErrorKind, MiddlewareError},
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
    router::Request,
};
use tokio::sync::Mutex;

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
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        let conn = match self.pool.acquire().await {
            Ok(pool_connection) => pool_connection,
            Err(err) => {
                log::error!("Failed to acquire a connection from the pool: {err}");

                return Err(MiddlewareError::new(err).into());
            }
        };

        let uow = SqlxUnitOfWork::<DB>::new(conn);

        request
            .context
            .insert("uow", Box::new(Arc::new(Mutex::new(uow))));

        Ok((request, EventReturn::Finish))
    }
}
