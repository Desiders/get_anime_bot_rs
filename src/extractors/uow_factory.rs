use crate::application::common::traits::UnitOfWorkFactory;

use std::sync::Arc;
use telers::{
    client::Bot, context::Context, errors::ExtractionError, extractors::FromEventAndContext,
    types::Update,
};

pub struct UoWFactoryWrapper<T: UnitOfWorkFactory>(pub T);

impl<T> FromEventAndContext for UoWFactoryWrapper<T>
where
    T: UnitOfWorkFactory + Clone + 'static,
{
    type Error = ExtractionError;

    fn extract(
        _bot: Arc<Bot>,
        _update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        let Some(value) = context.get("uow_factory") else {
            return Err(ExtractionError::new(concat!(
                "No found data in context by key `uow_factory`. ",
                "You didn't forget to add type to context? ",
            )));
        };

        match value.downcast_ref::<T>() {
            Some(value_ref) => Ok(UoWFactoryWrapper((*value_ref).clone())),
            None => Err(ExtractionError::new(concat!(
                "Data in context by key `uow_factory` has wrong type expected `",
                stringify!(T),
                "`. ",
                "You didn't forget to add type to context? ",
            ))),
        }
    }
}
