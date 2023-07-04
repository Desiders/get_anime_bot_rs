use async_trait::async_trait;
use sqlx::{self, Pool};
use telers::{
    error::{EventErrorKind, MiddlewareError},
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
    router::Request,
};

#[derive(Debug)]
pub struct DatabaseMiddleware<Database>
where
    Database: sqlx::Database,
{
    pool: sqlx::Pool<Database>,
}

impl<Database> DatabaseMiddleware<Database>
where
    Database: sqlx::Database,
{
    pub fn new(pool: Pool<Database>) -> Self {
        Self { pool }
    }
}

impl Clone for DatabaseMiddleware<sqlx::Postgres> {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

#[async_trait]
impl<Database, Client> Middleware<Client> for DatabaseMiddleware<Database>
where
    Database: sqlx::Database,
    Database::Connection: Sync,
    Client: Send + Sync + 'static,
{
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        let pool_connection = match self.pool.acquire().await {
            Ok(pool_connection) => pool_connection,
            Err(error) => {
                log::error!("Failed to acquire a connection from the pool: {error}");

                return Err(MiddlewareError::new(error).into());
            }
        };

        request
            .context
            .insert("pool_connection", Box::new(pool_connection));

        Ok((request, EventReturn::Finish))
    }
}
