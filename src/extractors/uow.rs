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

pub struct UnitOfWorkWrapper<UoWType>(Arc<Mutex<UoWType>>)
where
    UoWType: UnitOfWork;

impl<UoWType> UnitOfWorkWrapper<UoWType>
where
    UoWType: UnitOfWork,
{
    pub fn inner(&self) -> Arc<Mutex<UoWType>> {
        self.0.clone()
    }
}

impl<UoWType> From<Arc<Mutex<UoWType>>> for UnitOfWorkWrapper<UoWType>
where
    UoWType: UnitOfWork,
{
    fn from(inner: Arc<Mutex<UoWType>>) -> Self {
        Self(inner)
    }
}

from_context_into_impl!(
    [Client, UoWType: UnitOfWork],
    Arc<Mutex<UoWType>> => UnitOfWorkWrapper<UoWType>,
    "uow",
);
