use crate::application::common::traits::UnitOfWorkFactory;

use std::sync::Arc;
use telers::{
    client::Bot,
    context::Context,
    errors::ExtractionError,
    extractors::{from_context_into_impl, FromEventAndContext},
    types::Update,
};

pub struct UnitOfWorkFactoryWrapper<UoWFactory>(pub Arc<UoWFactory>)
where
    UoWFactory: UnitOfWorkFactory;

impl<UoW> From<Arc<UoW>> for UnitOfWorkFactoryWrapper<UoW>
where
    UoW: UnitOfWorkFactory,
{
    fn from(inner: Arc<UoW>) -> Self {
        Self(inner)
    }
}

from_context_into_impl!(
    [Client, UoWFactory: UnitOfWorkFactory], Arc<UoWFactory> => UnitOfWorkFactoryWrapper<UoWFactory>,
    "uow_factory",
);
