use crate::application::common::traits::UnitOfWorkFactory;

use async_trait::async_trait;
use telers::{
    errors::EventErrorKind,
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
    router::Request,
};

#[derive(Clone)]
pub struct Database<T> {
    uow_factory: T,
}

impl<T> Database<T> {
    pub const fn new(uow_factory: T) -> Self {
        Self { uow_factory }
    }
}

#[async_trait]
impl<T> Middleware for Database<T>
where
    T: UnitOfWorkFactory + Clone + Send + Sync + 'static,
{
    async fn call(&self, request: Request) -> Result<MiddlewareResponse, EventErrorKind> {
        request
            .context
            .insert("uow_factory", Box::new(self.uow_factory.clone()));

        Ok((request, EventReturn::Finish))
    }
}
