use crate::{
    application::{
        common::traits::UnitOfWork as _,
        user::{
            dto::{CreateUser, GetUserByTgId},
            traits::{UserReader as _, UserRepo as _},
        },
    },
    domain::user::entities::User as UserEntity,
    infrastructure::database::{
        repositories::{UserReaderImpl, UserRepoImpl},
        SqlxUnitOfWork,
    },
};

use anyhow::anyhow;
use async_trait::async_trait;
use log::{debug, error};
use sqlx::{Error, Postgres};
use std::marker::PhantomData;
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

pub struct ACLMiddleware<DB> {
    _phantom: PhantomData<DB>,
}

impl<DB> ACLMiddleware<DB> {
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl Clone for ACLMiddleware<Postgres> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Copy for ACLMiddleware<Postgres> {}

#[async_trait]
impl<Client> Middleware<Client> for ACLMiddleware<Postgres>
where
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
        let mut uow = if let Some(uow) = result.downcast_ref::<Mutex<SqlxUnitOfWork<Postgres>>>() {
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

        match UserReaderImpl::new(uow.connection())
            .get_by_tg_id(GetUserByTgId { tg_id: user.id })
            .await
        {
            Ok(db_user) => {
                debug!(target: module_path!(), "Successful get user: {db_user:?}");

                context.insert("db_user", Box::new(db_user));

                return Ok((request, EventReturn::Finish));
            }
            Err(err) => {
                if !matches!(err, Error::RowNotFound) {
                    error!(target: module_path!(), "Failed to get user by tg id `{tg_id}`: {err}", tg_id = user.id);

                    return Err(MiddlewareError::new(err).into());
                }
            }
        }

        let create_user = CreateUser {
            id: Uuid::new_v4(),
            tg_id: user.id,
            language_code: None,
            show_nsfw: None,
        };

        // Create user if not exists
        match UserRepoImpl::new(uow.connection())
            .create(create_user.clone())
            .await
        {
            Ok(_) => {
                if let Err(err) = uow.commit().await {
                    error!(target: module_path!(), "Failed to commit after create user with tg id `{tg_id}`: {err}", tg_id = user.id);
                } else {
                    debug!(target: module_path!(), "User with tg id `{tg_id}` created successful", tg_id = user.id);
                }

                let db_user = UserEntity {
                    id: create_user.id,
                    tg_id: create_user.tg_id,
                    language_code: create_user.language_code,
                    show_nsfw: create_user.show_nsfw,
                    created: OffsetDateTime::now_utc(), // approximate time
                };

                context.insert("db_user", Box::new(db_user));

                Ok((request, EventReturn::Finish))
            }
            Err(err) => Err(MiddlewareError::new(err).into()),
        }
    }
}
