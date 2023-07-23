use async_trait::async_trait;
use sqlx::{Database, Pool, Postgres};
use std::sync::Arc;
use telers::{
    error::{EventErrorKind, MiddlewareError},
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
    router::Request,
};
use tokio::sync::Mutex;

use crate::infrastructure::database::SqlxUnitOfWork;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct DatabaseMiddleware<DB>
where
    DB: Database,
{
    pool: Pool<DB>,
}

impl<DB> DatabaseMiddleware<DB>
where
    DB: Database,
{
    pub fn new(pool: Pool<DB>) -> Self {
        Self { pool }
    }
}

impl Clone for DatabaseMiddleware<Postgres> {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

#[async_trait]
impl<DB, Client> Middleware<Client> for DatabaseMiddleware<DB>
where
    DB: Database,
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
