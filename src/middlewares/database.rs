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
use tracing::{event, instrument, Level};

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
    #[instrument(skip(self, request))]
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        event!(Level::DEBUG, "Acquiring connection from the pool");

        let conn = match self.pool.acquire().await {
            Ok(pool_connection) => pool_connection,
            Err(err) => {
                event!(Level::ERROR, error = %err, "Failed to acquire connection from the pool");

                return Err(MiddlewareError::new(err).into());
            }
        };

        event!(Level::DEBUG, "Connection acquired");

        let uow = Arc::new(Mutex::new(SqlxUnitOfWork::<DB>::new(conn)));

        request.context.insert("uow", Box::new(uow));

        Ok((request, EventReturn::Finish))
    }
}
