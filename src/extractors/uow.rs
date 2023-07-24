use crate::application::common::traits::UnitOfWork;

use log::error;
use std::sync::Arc;
use telers::{
    client::Bot, context::Context, error::ExtractionError, extract::FromEventAndContext,
    types::Update,
};
use tokio::sync::Mutex;

/// Wrapper for foregein [`UnitOfWork`] to be used in [`FromEventAndContext`] extractor
pub struct UnitOfWorkWrapper<UnitOfWorkType>
where
    UnitOfWorkType: UnitOfWork,
{
    inner: Arc<Mutex<UnitOfWorkType>>,
}

impl<UnitOfWorkType> UnitOfWorkWrapper<UnitOfWorkType>
where
    UnitOfWorkType: UnitOfWork,
{
    pub fn new(inner: Arc<Mutex<UnitOfWorkType>>) -> Self {
        Self { inner }
    }

    /// Returns inner [`UnitOfWork`] wrapped in [`Arc`] and [`Mutex`]
    pub fn inner(&self) -> Arc<Mutex<UnitOfWorkType>> {
        self.inner.clone()
    }
}

impl<Client, UnitOfWorkType> FromEventAndContext<Client> for UnitOfWorkWrapper<UnitOfWorkType>
where
    UnitOfWorkType: UnitOfWork + 'static,
{
    type Error = ExtractionError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        let Some(result) = context.get("uow") else {
            return Err(ExtractionError::new("No unit of work found in context"));
        };

        let uow = if let Some(uow) = result.downcast_ref::<Arc<Mutex<UnitOfWorkType>>>() {
            uow.clone()
        } else {
            error!(
                target: module_path!(),
                "UnitOfWork in context is not a correct UnitOfWork"
            );

            return Err(ExtractionError::new(
                "UnitOfWork in context is not a correct UnitOfWork",
            ));
        };

        Ok(Self::new(uow))
    }
}
