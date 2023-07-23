use crate::{
    application::{
        common::{exceptions::RepoKind, traits::UnitOfWork},
        user::dto::{CreateUser, GetUserByTgId},
    },
    domain::user::entities::User as UserEntity,
};

use anyhow::anyhow;
use async_trait::async_trait;
use log::{debug, error};
use sqlx::PgConnection;
use std::{marker::PhantomData, sync::Arc};
use telers::{
    error::{EventErrorKind, MiddlewareError},
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
    router::Request,
    types::User,
};
use time::{self, OffsetDateTime};
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct ACLMiddleware<UnitOfWorkType> {
    _phantom: PhantomData<UnitOfWorkType>,
}

impl<UnitOfWorkType> ACLMiddleware<UnitOfWorkType> {
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<UnitOfWorkType> Clone for ACLMiddleware<UnitOfWorkType> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<UnitOfWorkType> Copy for ACLMiddleware<UnitOfWorkType> {}

#[async_trait]
impl<UnitOfWorkType, Client> Middleware<Client> for ACLMiddleware<UnitOfWorkType>
where
    for<'a> UnitOfWorkType:
        UnitOfWork<Connection<'a> = &'a mut PgConnection> + Send + Sync + 'static,
    Client: Send + Sync + 'static,
{
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        let context = request.context.clone();

        let Some(result) = context.get("event_user") else {
            error!(target: module_path!(), "No user found in context");

            return Err(MiddlewareError::new(anyhow!("No user found in context")).into());
        };
        let Some(user) = result.downcast_ref::<User>() else {
            error!(
                target: module_path!(),
                "User in context is not a correct User"
            );

            return Err(MiddlewareError::new(anyhow!("User in context is not a correct User")).into());
        };
        let Some(result) = context.get("uow") else {
            return Err(MiddlewareError::new(anyhow!("No unit of work found in context")).into());
        };
        let mut uow = if let Some(uow) = result.downcast_ref::<Arc<Mutex<UnitOfWorkType>>>() {
            uow.lock().await
        } else {
            error!(
                target: module_path!(),
                "UnitOfWork in context is not a correct UnitOfWork"
            );

            return Err(MiddlewareError::new(anyhow!(
                "UnitOfWork in context is not a correct UnitOfWork"
            ))
            .into());
        };

        match uow
            .user_reader()
            .get_by_tg_id(GetUserByTgId::new(user.id))
            .await
        {
            Ok(db_user) => {
                debug!(target: module_path!(), "Successful get user: {db_user:?}");

                context.insert("db_user", Box::new(db_user));

                return Ok((request, EventReturn::Finish));
            }
            Err(RepoKind::Exception(_)) => {
                debug!(target: module_path!(), "User with tg id `{tg_id}` not found", tg_id = user.id);
            }
            Err(RepoKind::Unexpected(err)) => {
                error!(target: module_path!(), "Failed to get user by tg id `{tg_id}`: {err}", tg_id = user.id);

                return Err(MiddlewareError::new(err).into());
            }
        }

        let create_user = CreateUser::new(Uuid::new_v4(), user.id, None, None);

        // Create user if not exists
        if let Err(err) = uow.user_repo().create(create_user.clone()).await {
            return Err(MiddlewareError::new(err).into());
        };

        if let Err(err) = uow.commit().await {
            error!(target: module_path!(), "Failed to commit after create user with tg id `{tg_id}`: {err}", tg_id = user.id);
        } else {
            debug!(target: module_path!(), "User with tg id `{tg_id}` created successful", tg_id = user.id);
        }

        let db_user = UserEntity {
            id: create_user.id(),
            tg_id: create_user.tg_id(),
            language_code: create_user.language_code().map(ToOwned::to_owned),
            show_nsfw: create_user.show_nsfw(),
            created: OffsetDateTime::now_utc(), // approximate time
        };

        context.insert("db_user", Box::new(db_user));

        Ok((request, EventReturn::Finish))
    }
}
