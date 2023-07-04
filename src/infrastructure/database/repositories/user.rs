use super::base::Repo;
use crate::{
    application::user::{
        dto::{
            CreateUser, GetUserById, GetUserByTgId, UpdateUserLanguageCode, UpdateUserShowNsfw,
            User,
        },
        traits::{UserReader, UserRepo},
    },
    infrastructure::database::models::{User as UserModel, UserTable},
};

use async_trait::async_trait;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder as _;
use sqlx::{pool::PoolConnection, Error, Postgres};

pub struct UserRepoImpl<Executor> {
    executor: Executor,
}

impl<Executor> Repo<Executor> for UserRepoImpl<Executor> {
    fn new(executor: Executor) -> Self {
        Self { executor }
    }
}

#[async_trait]
impl UserRepo<Postgres> for UserRepoImpl<PoolConnection<Postgres>> {
    type CreateError = Error;
    type UpdateLanguageCodeError = Error;
    type UpdateShowNsfwError = Error;

    async fn create(&mut self, user: CreateUser) -> Result<(), Self::CreateError> {
        let (sql, values) = Query::insert()
            .into_table(UserTable::Table)
            .columns(vec![
                UserTable::Id,
                UserTable::TgId,
                UserTable::LanguageCode,
                UserTable::ShowNsfw,
            ])
            .values_panic([
                user.id.into(),
                user.tg_id.into(),
                user.language_code.into(),
                user.show_nsfw.into(),
            ])
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut self.executor)
            .await
            .map(|_| ())
    }

    async fn update_language_code(
        &mut self,
        user: UpdateUserLanguageCode,
    ) -> Result<(), Self::UpdateLanguageCodeError> {
        let (sql, values) = Query::update()
            .table(UserTable::Table)
            .values([(UserTable::LanguageCode, user.language_code.into())])
            .and_where(Expr::col(UserTable::Id).is(user.id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut self.executor)
            .await
            .map(|_| ())
    }

    async fn update_show_nsfw(
        &mut self,
        user: UpdateUserShowNsfw,
    ) -> Result<(), Self::UpdateShowNsfwError> {
        let (sql, values) = Query::update()
            .table(UserTable::Table)
            .values([(UserTable::ShowNsfw, user.show_nsfw.into())])
            .and_where(Expr::col(UserTable::Id).is(user.id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut self.executor)
            .await
            .map(|_| ())
    }
}

pub struct UserReaderImpl<Executor> {
    executor: Executor,
}

impl<Executor> Repo<Executor> for UserReaderImpl<Executor> {
    fn new(executor: Executor) -> Self {
        Self { executor }
    }
}

#[async_trait]
impl UserReader<Postgres> for UserReaderImpl<PoolConnection<Postgres>> {
    type GetError = Error;
    type GetByIdError = Error;

    async fn get_by_id(&mut self, user: GetUserById) -> Result<User, Self::GetError> {
        let (sql, values) = Query::select()
            .columns([
                UserTable::Id,
                UserTable::TgId,
                UserTable::LanguageCode,
                UserTable::ShowNsfw,
                UserTable::Created,
            ])
            .from(UserTable::Table)
            .and_where(Expr::col(UserTable::Id).is(user.id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut self.executor)
            .await
            .map(|user_model: UserModel| user_model.into())
    }

    async fn get_by_tg_id(&mut self, user: GetUserByTgId) -> Result<User, Self::GetByIdError> {
        let (sql, values) = Query::select()
            .columns([
                UserTable::Id,
                UserTable::TgId,
                UserTable::LanguageCode,
                UserTable::ShowNsfw,
                UserTable::Created,
            ])
            .from(UserTable::Table)
            .and_where(Expr::col(UserTable::TgId).is(user.tg_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut self.executor)
            .await
            .map(|user_model: UserModel| user_model.into())
    }
}
