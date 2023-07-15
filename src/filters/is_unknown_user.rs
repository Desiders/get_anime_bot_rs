use crate::{
    application::{
        common::traits::UnitOfWork as _,
        user::{dto::GetUserByTgId, traits::UserReader as _},
    },
    infrastructure::database::{repositories::UserReaderImpl, SqlxUnitOfWork},
};

use async_trait::async_trait;
use log::error;
use sqlx::Postgres;
use std::marker::PhantomData;
use telers::{
    client::Bot,
    context::Context,
    filters::Filter,
    types::{Update, User},
};
use tokio::sync::Mutex;

pub struct IsUnknownUser<DB> {
    _phantom: PhantomData<DB>,
}

impl<DB> IsUnknownUser<DB> {
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<Client> Filter<Client> for IsUnknownUser<Postgres> {
    async fn check(&self, _bot: &Bot<Client>, _update: &Update, context: &Context) -> bool {
        let Some(result) = context.get("event_user") else {
            error!(target: module_path!(), "No user found in context");

            return false;
        };
        let Some(user) = result.downcast_ref::<User>() else {
            error!(
                target: module_path!(),
                "User in context is not a correct User"
            );

            return false;
        };
        let Some(result) = context.get("uow") else {
            error!(target: module_path!(), "No unit of work found in context");

            return false;
        };
        let mut uow = match result.downcast_ref::<Mutex<SqlxUnitOfWork<Postgres>>>() {
            Some(uow) => uow.lock().await,
            None => {
                error!(
                    target: module_path!(),
                    "UnitOfWork in context is not a correct UnitOfWork"
                );

                return false;
            }
        };

        UserReaderImpl::new(uow.connection())
            .get_by_tg_id(GetUserByTgId { tg_id: user.id })
            .await
            .is_ok()
    }
}
