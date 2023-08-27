use crate::application::common::traits::UnitOfWork;

use std::sync::Arc;
use telers::{
    client::Bot,
    context::Context,
    errors::ExtractionError,
    extractors::{from_context_into_impl, FromEventAndContext},
    types::Update,
};
use tokio::sync::Mutex;

pub struct UnitOfWorkWrapper<UoW>(pub Arc<Mutex<UoW>>)
where
    UoW: UnitOfWork;

impl<UoW> UnitOfWorkWrapper<UoW>
where
    UoW: UnitOfWork,
{
    pub fn inner(&self) -> Arc<Mutex<UoW>> {
        self.0.clone()
    }
}

impl<UoW> From<Arc<Mutex<UoW>>> for UnitOfWorkWrapper<UoW>
where
    UoW: UnitOfWork,
{
    fn from(inner: Arc<Mutex<UoW>>) -> Self {
        Self(inner)
    }
}

from_context_into_impl!(
    [Client, UoW: UnitOfWork],
    Arc<Mutex<UoW>> => UnitOfWorkWrapper<UoW>,
    "uow",
);
