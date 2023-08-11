use crate::{
    application::{
        common::{exceptions::RepoKind, traits::UnitOfWork},
        user::dto::{CreateUser, GetUserByTgId},
    },
    domain::user::entities::User as UserEntity,
};

use anyhow::anyhow;
use async_trait::async_trait;
use sqlx::PgConnection;
use std::{marker::PhantomData, sync::Arc};
use telers::{
    errors::{EventErrorKind, MiddlewareError},
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
    router::Request,
};
use time::{self, OffsetDateTime};
use tokio::sync::Mutex;
use tracing::{event, field, instrument, Level};
use uuid::Uuid;

pub struct Acl<UnitOfWorkType> {
    _phantom: PhantomData<UnitOfWorkType>,
}

impl<UnitOfWorkType> Acl<UnitOfWorkType> {
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<UnitOfWorkType> Clone for Acl<UnitOfWorkType> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<UnitOfWorkType> Copy for Acl<UnitOfWorkType> {}

#[async_trait]
impl<UnitOfWorkType, Client> Middleware<Client> for Acl<UnitOfWorkType>
where
    for<'a> UnitOfWorkType:
        UnitOfWork<Connection<'a> = &'a mut PgConnection> + Send + Sync + 'static,
    Client: Send + Sync + 'static,
{
    #[instrument(skip(self, request))]
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        let context = request.context.clone();

        let Some(user) = request.update.user() else {
            event!(Level::DEBUG,"No user found in update");

            return Ok((request, EventReturn::Skip));
        };

        let Some(result) = context.get("uow") else {
            return Err(MiddlewareError::new(anyhow!("No unit of work found in context")).into());
        };
        let mut uow = if let Some(uow) = result.downcast_ref::<Arc<Mutex<UnitOfWorkType>>>() {
            uow.lock().await
        } else {
            event!(
                Level::ERROR,
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
                event!(
                    Level::DEBUG,
                    user_id = field::display(db_user.id),
                    tg_id = db_user.tg_id,
                    "Successful get user",
                );

                context.insert("db_user", Box::new(db_user));

                return Ok((request, EventReturn::Finish));
            }
            Err(RepoKind::Exception(_)) => {
                event!(Level::DEBUG, tg_id = user.id, "User not found");
            }
            Err(RepoKind::Unexpected(err)) => {
                event!(Level::ERROR,
                    error = %err,
                    tg_id = user.id,
                    "Failed to get user",
                );

                return Err(MiddlewareError::new(err).into());
            }
        }

        let create_user = CreateUser::new(Uuid::new_v4(), user.id, None, None);

        // Create user if not exists
        if let Err(err) = uow.user_repo().create(create_user.clone()).await {
            event!(Level::ERROR,
                error = %err,
                ?create_user,
                "Failed to create user"
            );

            return Err(MiddlewareError::new(err).into());
        } else {
            event!(Level::DEBUG, tg_id = user.id, "User created successful");
        };

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
