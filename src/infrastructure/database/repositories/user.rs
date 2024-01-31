use crate::{
    application::{
        common::exceptions::{RepoError, RepoKind},
        user::{
            dto::{
                CreateUser, GetUserById, GetUserByTgId, UpdateUserLanguageCode, UpdateUserShowNsfw,
            },
            exceptions::{UserIdNotExist, UserTgIdAlreadyExists, UserTgIdNotExist},
            traits::{UserReader, UserRepo},
        },
    },
    domain::user::entities::User,
    infrastructure::database::models::User as UserModel,
};

use async_trait::async_trait;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder as _;
use sqlx::PgConnection;

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
    async fn create<'s>(
        &mut self,
        user: CreateUser<'s>,
    ) -> Result<(), RepoKind<UserTgIdAlreadyExists>> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("users"))
            .columns([
                Alias::new("id"),
                Alias::new("tg_id"),
                Alias::new("language_code"),
                Alias::new("show_nsfw"),
            ])
            .values_panic([
                (*user.id()).into(),
                user.tg_id().into(),
                user.language_code().into(),
                user.show_nsfw().into(),
            ])
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut *self.conn)
            .await
            .map(|_| ())
            .map_err(|err| {
                if let sqlx::Error::Database(ref err) = err {
                    if let Some(code) = err.code() {
                        if code == "23505" {
                            return RepoKind::exception(UserTgIdAlreadyExists::new(
                                user.tg_id(),
                                err.to_string(),
                            ));
                        }
                    }
                }
                RepoKind::unexpected(err)
            })
    }

    async fn update_language_code<'s>(
        &mut self,
        user: UpdateUserLanguageCode<'s>,
    ) -> Result<(), RepoError> {
        let (sql, values) = Query::update()
            .table(Alias::new("users"))
            .values([(Alias::new("language_code"), user.language_code().into())])
            .and_where(Expr::col(Alias::new("id")).eq(*user.id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut *self.conn)
            .await
            .map(|_| ())
            .map_err(Into::into)
    }

    async fn update_show_nsfw<'s>(
        &mut self,
        user: UpdateUserShowNsfw<'s>,
    ) -> Result<(), RepoError> {
        let (sql, values) = Query::update()
            .table(Alias::new("users"))
            .values([(Alias::new("show_nsfw"), user.show_nsfw().into())])
            .and_where(Expr::col(Alias::new("id")).eq(*user.id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut *self.conn)
            .await
            .map(|_| ())
            .map_err(Into::into)
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
    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_id<'s>(
        &mut self,
        user: GetUserById<'s>,
    ) -> Result<User, RepoKind<UserIdNotExist>> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("tg_id"),
                Alias::new("language_code"),
                Alias::new("show_nsfw"),
                Alias::new("created"),
            ])
            .from(Alias::new("users"))
            .and_where(Expr::col(Alias::new("id")).eq(*user.id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|user_model: UserModel| user_model.into())
            .map_err(|err| {
                if let sqlx::Error::RowNotFound = err {
                    return RepoKind::exception(UserIdNotExist::new(*user.id(), err.to_string()));
                }
                RepoKind::unexpected(err)
            })
    }

    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_tg_id(
        &mut self,
        user: GetUserByTgId,
    ) -> Result<User, RepoKind<UserTgIdNotExist>> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("tg_id"),
                Alias::new("language_code"),
                Alias::new("show_nsfw"),
                Alias::new("created"),
            ])
            .from(Alias::new("users"))
            .and_where(Expr::col(Alias::new("tg_id")).eq(user.tg_id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|user_model: UserModel| user_model.into())
            .map_err(|err| {
                if let sqlx::Error::RowNotFound = err {
                    return RepoKind::exception(UserTgIdNotExist::new(
                        user.tg_id(),
                        err.to_string(),
                    ));
                }
                RepoKind::unexpected(err)
            })
    }
}
