use crate::{
    application::{
        common::{
            exceptions::RepoKind,
            traits::{UnitOfWork, UnitOfWorkFactory},
        },
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
use tracing::{event, field, Level};
use uuid::Uuid;

#[allow(clippy::upper_case_acronyms)]
pub struct ACL<UoWFactory> {
    _phantom: PhantomData<UoWFactory>,
}

impl<UoWFactory> ACL<UoWFactory> {
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<UoWFactory> Clone for ACL<UoWFactory> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<UoWFactory> Copy for ACL<UoWFactory> {}

#[async_trait]
impl<UoWFactory, Client> Middleware<Client> for ACL<UoWFactory>
where
    UoWFactory: UnitOfWorkFactory + Send + Sync + 'static,
    for<'a> UoWFactory::UnitOfWork:
        UnitOfWork<Connection<'a> = &'a mut PgConnection> + Send + Sync + 'static,
    Client: Send + Sync + 'static,
{
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        let context = request.context.clone();

        let Some(user_id) = request.update.from_id() else {
            event!(Level::DEBUG, "No user found in update");

            return Ok((request, EventReturn::Skip));
        };

        let Some(result) = context.get("uow_factory") else {
            return Err(
                MiddlewareError::new(anyhow!("No unit of work factory found in context")).into(),
            );
        };
        let Some(uow_factory) = result.downcast_ref::<Arc<UoWFactory>>() else {
            event!(
                Level::ERROR,
                "Unit of work factory in context is not a correct `Arc<impl UnitOfWorkFactory>`"
            );

            return Err(MiddlewareError::new(anyhow!(
                "Unit of work factory in context is not a correct `Arc<impl UnitOfWorkFactory>`"
            ))
            .into());
        };

        let mut uow = uow_factory.new_unit_of_work();

        let get_user_result = uow
            .user_reader()
            .await
            .map_err(MiddlewareError::new)?
            .get_by_tg_id(GetUserByTgId::new(user_id))
            .await;

        match get_user_result {
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
            Err(RepoKind::Exception(_)) => {}
            Err(RepoKind::Unexpected(err)) => {
                event!(Level::ERROR,
                    error = %err,
                    tg_id = user_id,
                    "Failed to get user",
                );

                return Err(MiddlewareError::new(err).into());
            }
        }

        event!(Level::DEBUG, tg_id = user_id, "User not found");

        let create_user = CreateUser::new(Uuid::new_v4(), user_id, None, None);

        let create_user_result = uow
            .user_repo()
            .await
            .map_err(MiddlewareError::new)?
            .create(create_user.clone())
            .await;

        if let Err(err) = create_user_result {
            uow.rollback().await.map_err(MiddlewareError::new)?;

            event!(Level::ERROR,
                error = %err,
                ?create_user,
                "Failed to create user"
            );

            return Err(MiddlewareError::new(err).into());
        }

        uow.commit().await.map_err(MiddlewareError::new)?;

        event!(Level::DEBUG, tg_id = user_id, "User created successful");

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
