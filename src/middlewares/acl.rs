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
use std::marker::PhantomData;
use telers::{
    errors::{EventErrorKind, MiddlewareError},
    event::EventReturn,
    middlewares::outer::{Middleware, MiddlewareResponse},
    router::Request,
};
use time::{self, OffsetDateTime};
use tracing::{event, field, instrument, Level, Span};
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
        *self
    }
}

impl<UoWFactory> Copy for ACL<UoWFactory> {}

#[async_trait]
impl<UoWFactory> Middleware for ACL<UoWFactory>
where
    UoWFactory: UnitOfWorkFactory + Send + Sync + 'static,
    for<'a> UoWFactory::UnitOfWork:
        UnitOfWork<Connection<'a> = &'a mut PgConnection> + Send + Sync + 'static,
{
    #[instrument(skip_all, fields(user_id))]
    async fn call(&self, request: Request) -> Result<MiddlewareResponse, EventErrorKind> {
        let context = request.context.clone();

        let Some(user_id) = request.update.from_id() else {
            event!(Level::DEBUG, "No user found in update");

            return Ok((request, EventReturn::Skip));
        };

        Span::current().record("user_id", user_id);

        let Some(result) = context.get("uow_factory") else {
            return Err(
                MiddlewareError::new(anyhow!("No unit of work factory found in context")).into(),
            );
        };
        let Some(uow_factory) = result.downcast_ref::<UoWFactory>() else {
            return Err(MiddlewareError::new(anyhow!(
                "Unit of work factory in context is not a correct `Arc<impl UnitOfWorkFactory>`"
            ))
            .into());
        };

        let mut uow = uow_factory.new_unit_of_work();

        // We need to drop the result to release the lock on the context,
        // because without it inserting value can cause a deadlock
        drop(result);

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
                    db_user_id = field::display(db_user.id),
                    "Successful get user",
                );

                context.insert("db_user", Box::new(db_user));

                return Ok((request, EventReturn::Finish));
            }
            Err(RepoKind::Exception(_)) => {}
            Err(RepoKind::Unexpected(err)) => {
                event!(Level::ERROR, %err, "Failed to get user");

                return Err(MiddlewareError::new(err).into());
            }
        }

        event!(Level::DEBUG, "User not found");

        let db_user_id = Uuid::new_v4();

        let create_user = CreateUser::new(&db_user_id, user_id, None, None);

        let create_user_result = uow
            .user_repo()
            .await
            .map_err(MiddlewareError::new)?
            .create(create_user.clone())
            .await;

        if let Err(err) = create_user_result {
            uow.rollback().await.map_err(MiddlewareError::new)?;

            event!(Level::ERROR,
                %err,
                ?create_user,
                "Failed to create user"
            );

            return Err(MiddlewareError::new(err).into());
        }

        uow.commit().await.map_err(MiddlewareError::new)?;

        event!(Level::DEBUG, "User created successful");

        let db_user = UserEntity {
            id: *create_user.id(),
            tg_id: create_user.tg_id(),
            language_code: create_user.language_code().map(ToOwned::to_owned),
            show_nsfw: create_user.show_nsfw(),
            created: OffsetDateTime::now_utc(), // approximate time
        };

        context.insert("db_user", Box::new(db_user));

        Ok((request, EventReturn::Finish))
    }
}
