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
use sqlx::{Error, PgConnection};

pub struct UserRepoImpl<Conn> {
    conn: Conn,
}

impl<Conn> UserRepoImpl<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> UserRepo for UserRepoImpl<&'a mut PgConnection> {
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
            .execute(&mut *self.conn)
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
            .and_where(Expr::col(UserTable::Id).eq(user.id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut *self.conn)
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
            .and_where(Expr::col(UserTable::Id).eq(user.id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut *self.conn)
            .await
            .map(|_| ())
    }
}

pub struct UserReaderImpl<Conn> {
    conn: Conn,
}

impl<Conn> UserReaderImpl<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> UserReader for UserReaderImpl<&'a mut PgConnection> {
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
            .and_where(Expr::col(UserTable::Id).eq(user.id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
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
            .and_where(Expr::col(UserTable::TgId).eq(user.tg_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|user_model: UserModel| user_model.into())
    }
}
