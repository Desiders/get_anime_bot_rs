use crate::{
    application::user_media_view::{
        dto::{
            CreateUserMediaView, GetUserMediaViewById, GetUserMediaViewByMediaAgeRestriction,
            GetUserMediaViewByMediaGenre, GetUserMediaViewByMediaId,
            GetUserMediaViewByMediaSourceId, GetUserMediaViewByMediaType, GetUserMediaViewByUserId,
            GetUserMediaViewByUserTgId,
        },
        traits::{UserMediaViewReader, UserMediaViewRepo},
    },
    domain::user_media_view::entities::UserMediaView,
    infrastructure::database::models::UserMediaView as UserMediaViewModel,
};

use async_trait::async_trait;
use sea_query::{Alias, Expr, JoinType, PostgresQueryBuilder, Query};
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

    async fn get_by_user_tg_id(
        &mut self,
        user_media_view: GetUserMediaViewByUserTgId,
    ) -> Result<Vec<UserMediaView>, Self::GetByUserIdError> {
        let (sql, values) = Query::select()
            .columns([
                (Alias::new("user_media_views"), Alias::new("id")),
                (Alias::new("user_media_views"), Alias::new("user_id")),
                (Alias::new("user_media_views"), Alias::new("media_id")),
                (Alias::new("user_media_views"), Alias::new("created")),
            ])
            .from(Alias::new("user_media_views"))
            .join(
                JoinType::RightJoin,
                Alias::new("users"),
                Expr::col((Alias::new("users"), Alias::new("tg_id")))
                    .eq(user_media_view.user_tg_id()),
            )
            .and_where(
                Expr::col((Alias::new("users"), Alias::new("id")))
                    .equals((Alias::new("user_media_views"), Alias::new("user_id"))),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
    }

    async fn get_by_media_genre(
        &mut self,
        user_media_view: GetUserMediaViewByMediaGenre,
    ) -> Result<Vec<UserMediaView>, Self::GetByMediaIdError> {
        let (sql, values) = Query::select()
            .columns([
                (Alias::new("user_media_views"), Alias::new("id")),
                (Alias::new("user_media_views"), Alias::new("user_id")),
                (Alias::new("user_media_views"), Alias::new("media_id")),
                (Alias::new("user_media_views"), Alias::new("created")),
            ])
            .from(Alias::new("user_media_views"))
            .join(
                JoinType::RightJoin,
                Alias::new("media"),
                Expr::col((Alias::new("media"), Alias::new("genre"))).eq(user_media_view.genre()),
            )
            .and_where(
                Expr::col((Alias::new("media"), Alias::new("id")))
                    .equals((Alias::new("user_media_views"), Alias::new("media_id"))),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
    }

    async fn get_by_media_type(
        &mut self,
        user_media_view: GetUserMediaViewByMediaType,
    ) -> Result<Vec<UserMediaView>, Self::GetByMediaIdError> {
        let (sql, values) = Query::select()
            .columns([
                (Alias::new("user_media_views"), Alias::new("id")),
                (Alias::new("user_media_views"), Alias::new("user_id")),
                (Alias::new("user_media_views"), Alias::new("media_id")),
                (Alias::new("user_media_views"), Alias::new("created")),
            ])
            .from(Alias::new("user_media_views"))
            .join(
                JoinType::RightJoin,
                Alias::new("media"),
                Expr::col((Alias::new("media"), Alias::new("media_type")))
                    .eq(user_media_view.media_type()),
            )
            .and_where(
                Expr::col((Alias::new("media"), Alias::new("id")))
                    .equals((Alias::new("user_media_views"), Alias::new("media_id"))),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
    }

    async fn get_by_media_age_restriction(
        &mut self,
        user_media_view: GetUserMediaViewByMediaAgeRestriction,
    ) -> Result<Vec<UserMediaView>, Self::GetByMediaIdError> {
        let (sql, values) = Query::select()
            .columns([
                (Alias::new("user_media_views"), Alias::new("id")),
                (Alias::new("user_media_views"), Alias::new("user_id")),
                (Alias::new("user_media_views"), Alias::new("media_id")),
                (Alias::new("user_media_views"), Alias::new("created")),
            ])
            .from(Alias::new("user_media_views"))
            .join(
                JoinType::RightJoin,
                Alias::new("media"),
                Expr::col((Alias::new("media"), Alias::new("is_sfw"))).eq(user_media_view.is_sfw()),
            )
            .and_where(
                Expr::col((Alias::new("media"), Alias::new("id")))
                    .equals((Alias::new("user_media_views"), Alias::new("media_id"))),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
    }

    async fn get_by_media_source_id(
        &mut self,
        user_media_view: GetUserMediaViewByMediaSourceId,
    ) -> Result<Vec<UserMediaView>, Self::GetByMediaIdError> {
        let (sql, values) = Query::select()
            .columns([
                (Alias::new("user_media_views"), Alias::new("id")),
                (Alias::new("user_media_views"), Alias::new("user_id")),
                (Alias::new("user_media_views"), Alias::new("media_id")),
                (Alias::new("user_media_views"), Alias::new("created")),
            ])
            .from(Alias::new("user_media_views"))
            .join(
                JoinType::RightJoin,
                Alias::new("media"),
                Expr::col((Alias::new("media"), Alias::new("source_id")))
                    .eq(user_media_view.source_id()),
            )
            .and_where(
                Expr::col((Alias::new("media"), Alias::new("id")))
                    .equals((Alias::new("user_media_views"), Alias::new("media_id"))),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
    }
}
