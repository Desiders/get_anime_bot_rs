use crate::{
    application::user_media_view::{
        dto::{
            CreateUserMediaView, GetUserMediaViewById, GetUserMediaViewByMediaId,
            GetUserMediaViewByUserId,
        },
        traits::{UserMediaViewReader, UserMediaViewRepo},
    },
    domain::user_media_view::entities::UserMediaView,
    infrastructure::database::models::UserMediaView as UserMediaViewModel,
};

use async_trait::async_trait;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder as _;
use sqlx::{Error, PgConnection};

#[allow(clippy::module_name_repetitions)]
pub struct UserMediaViewRepoImpl<Conn> {
    conn: Conn,
}

impl<Conn> UserMediaViewRepoImpl<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> UserMediaViewRepo for UserMediaViewRepoImpl<&'a mut PgConnection> {
    type CreateError = Error;

    async fn create(
        &mut self,
        user_media_view: CreateUserMediaView,
    ) -> Result<(), Self::CreateError> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("user_media_views"))
            .columns(vec![
                Alias::new("id"),
                Alias::new("user_id"),
                Alias::new("media_id"),
            ])
            .values_panic([
                user_media_view.id().into(),
                user_media_view.user_id().into(),
                user_media_view.media_id().into(),
            ])
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut *self.conn)
            .await
            .map(|_| ())
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct UserMediaViewReaderImpl<Conn> {
    conn: Conn,
}

impl<Conn> UserMediaViewReaderImpl<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> UserMediaViewReader for UserMediaViewReaderImpl<&'a mut PgConnection> {
    type GetError = Error;
    type GetByIdError = Error;
    type GetByUserIdError = Error;
    type GetByMediaIdError = Error;

    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_id(
        &mut self,
        user: GetUserMediaViewById,
    ) -> Result<UserMediaView, Self::GetError> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("user_id"),
                Alias::new("media_id"),
                Alias::new("created"),
            ])
            .from(Alias::new("user_media_views"))
            .and_where(Expr::col(Alias::new("id")).eq(user.id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|user_media_view_model: UserMediaViewModel| user_media_view_model.into())
    }

    async fn get_by_user_id(
        &mut self,
        user_media_view: GetUserMediaViewByUserId,
    ) -> Result<Vec<UserMediaView>, Self::GetByUserIdError> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("user_id"),
                Alias::new("media_id"),
                Alias::new("created"),
            ])
            .from(Alias::new("user_media_views"))
            .and_where(Expr::col(Alias::new("user_id")).eq(user_media_view.user_id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
    }

    async fn get_by_media_id(
        &mut self,
        user_media_view: GetUserMediaViewByMediaId,
    ) -> Result<Vec<UserMediaView>, Self::GetByMediaIdError> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("user_id"),
                Alias::new("media_id"),
                Alias::new("created"),
            ])
            .from(Alias::new("user_media_views"))
            .and_where(Expr::col(Alias::new("media_id")).eq(user_media_view.media_id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
    }
}
