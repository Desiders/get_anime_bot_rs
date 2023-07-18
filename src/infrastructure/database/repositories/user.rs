use crate::{
    application::user::{
        dto::{
            CreateUser, GetUserById, GetUserByTgId, UpdateUserLanguageCode, UpdateUserShowNsfw,
            User,
        },
        traits::{UserReader, UserRepo},
    },
    infrastructure::database::models::User as UserModel,
};

use async_trait::async_trait;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder as _;
use sqlx::{Error, PgConnection};

#[allow(clippy::module_name_repetitions)]
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
            .into_table(Alias::new("users"))
            .columns(vec![
                Alias::new("id"),
                Alias::new("tg_id"),
                Alias::new("language_code"),
                Alias::new("show_nsfw"),
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
            .table(Alias::new("users"))
            .values([(Alias::new("language_code"), user.language_code.into())])
            .and_where(Expr::col(Alias::new("id")).eq(user.id))
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
            .table(Alias::new("users"))
            .values([(Alias::new("show_nsfw"), user.show_nsfw.into())])
            .and_where(Expr::col(Alias::new("id")).eq(user.id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut *self.conn)
            .await
            .map(|_| ())
    }
}

#[allow(clippy::module_name_repetitions)]
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

    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_id(&mut self, user: GetUserById) -> Result<User, Self::GetError> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("tg_id"),
                Alias::new("language_code"),
                Alias::new("show_nsfw"),
                Alias::new("created"),
            ])
            .from(Alias::new("users"))
            .and_where(Expr::col(Alias::new("id")).eq(user.id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|user_model: UserModel| user_model.into())
    }

    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_tg_id(&mut self, user: GetUserByTgId) -> Result<User, Self::GetByIdError> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("tg_id"),
                Alias::new("language_code"),
                Alias::new("show_nsfw"),
                Alias::new("created"),
            ])
            .from(Alias::new("users"))
            .and_where(Expr::col(Alias::new("tg_id")).eq(user.tg_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|user_model: UserModel| user_model.into())
    }
}
