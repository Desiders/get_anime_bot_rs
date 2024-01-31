use crate::application::common::traits::UnitOfWorkFactory;

use async_trait::async_trait;
use telers::{
    errors::EventErrorKind,
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
    router::Request,
};

#[derive(Clone)]
pub struct Database<UoWFactory> {
    uow_factory: UoWFactory,
}

impl<UoWFactory> Database<UoWFactory> {
    pub const fn new(uow_factory: UoWFactory) -> Self {
        Self { uow_factory }
    }
}

#[async_trait]
impl<UoWFactory, Client> Middleware<Client> for Database<UoWFactory>
where
    UoWFactory: UnitOfWorkFactory + Clone + Send + Sync + 'static,
    Client: Send + Sync + 'static,
{
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
