use crate::{
    application::{
        common::exceptions::{RepoError, RepoKind},
        user_media_view::{
            dto::{
                CreateUserMediaView, GetUserMediaViewById, GetUserMediaViewByMediaAgeRestriction,
                GetUserMediaViewByMediaGenre, GetUserMediaViewByMediaId,
                GetUserMediaViewByMediaSourceId, GetUserMediaViewByMediaType,
                GetUserMediaViewByUserId, GetUserMediaViewByUserTgId,
            },
            exceptions::{UserMediaViewIdNotExist, UserMediaViewUserIdAndMediaIdAlreadyExists},
            traits::{UserMediaViewReader, UserMediaViewRepo},
        },
    },
    domain::user_media_view::entities::UserMediaView,
    infrastructure::database::models::UserMediaView as UserMediaViewModel,
};

use async_trait::async_trait;
use sea_query::{Alias, Expr, JoinType, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder as _;
use sqlx::PgConnection;

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
    async fn create<'s>(
        &mut self,
        user_media_view: CreateUserMediaView<'s>,
    ) -> Result<(), RepoKind<UserMediaViewUserIdAndMediaIdAlreadyExists>> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("user_media_views"))
            .columns([
                Alias::new("id"),
                Alias::new("user_id"),
                Alias::new("media_id"),
            ])
            .values_panic([
                (*user_media_view.id()).into(),
                (*user_media_view.user_id()).into(),
                (*user_media_view.media_id()).into(),
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
                            return RepoKind::exception(
                                UserMediaViewUserIdAndMediaIdAlreadyExists::new(
                                    *user_media_view.user_id(),
                                    *user_media_view.media_id(),
                                    err.to_string(),
                                ),
                            );
                        }
                    }
                }
                RepoKind::unexpected(err)
            })
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
    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_id<'s>(
        &mut self,
        user_media_view: GetUserMediaViewById<'s>,
    ) -> Result<UserMediaView, RepoKind<UserMediaViewIdNotExist>> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("user_id"),
                Alias::new("media_id"),
                Alias::new("created"),
            ])
            .from(Alias::new("user_media_views"))
            .and_where(Expr::col(Alias::new("id")).eq(*user_media_view.id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|user_media_view_model: UserMediaViewModel| user_media_view_model.into())
            .map_err(|err| {
                if let sqlx::Error::RowNotFound = err {
                    RepoKind::exception(UserMediaViewIdNotExist::new(
                        *user_media_view.id(),
                        err.to_string(),
                    ))
                } else {
                    RepoKind::unexpected(err)
                }
            })
    }

    async fn get_by_user_id<'s>(
        &mut self,
        user_media_view: GetUserMediaViewByUserId<'s>,
    ) -> Result<Vec<UserMediaView>, RepoError> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("user_id"),
                Alias::new("media_id"),
                Alias::new("created"),
            ])
            .from(Alias::new("user_media_views"))
            .and_where(Expr::col(Alias::new("user_id")).eq(*user_media_view.user_id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
            .map_err(Into::into)
    }

    async fn get_by_media_id<'s>(
        &mut self,
        user_media_view: GetUserMediaViewByMediaId<'s>,
    ) -> Result<Vec<UserMediaView>, RepoError> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("user_id"),
                Alias::new("media_id"),
                Alias::new("created"),
            ])
            .from(Alias::new("user_media_views"))
            .and_where(Expr::col(Alias::new("media_id")).eq(*user_media_view.media_id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
            .map_err(Into::into)
    }

    async fn get_by_user_tg_id(
        &mut self,
        user_media_view: GetUserMediaViewByUserTgId,
    ) -> Result<Vec<UserMediaView>, RepoError> {
        let (sql, values) = Query::select()
            .columns([
                (Alias::new("user_media_views"), Alias::new("id")),
                (Alias::new("user_media_views"), Alias::new("user_id")),
                (Alias::new("user_media_views"), Alias::new("media_id")),
                (Alias::new("user_media_views"), Alias::new("created")),
            ])
            .from(Alias::new("user_media_views"))
            .join(
                JoinType::InnerJoin,
                Alias::new("users"),
                Expr::col((Alias::new("users"), Alias::new("tg_id")))
                    .eq(user_media_view.user_tg_id())
                    .and(
                        Expr::col((Alias::new("users"), Alias::new("id")))
                            .equals((Alias::new("user_media_views"), Alias::new("user_id"))),
                    ),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
            .map_err(Into::into)
    }

    async fn get_by_media_genre<'s>(
        &mut self,
        user_media_view: GetUserMediaViewByMediaGenre<'s>,
    ) -> Result<Vec<UserMediaView>, RepoError> {
        let (sql, values) = Query::select()
            .columns([
                (Alias::new("user_media_views"), Alias::new("id")),
                (Alias::new("user_media_views"), Alias::new("user_id")),
                (Alias::new("user_media_views"), Alias::new("media_id")),
                (Alias::new("user_media_views"), Alias::new("created")),
            ])
            .from(Alias::new("user_media_views"))
            .join(
                JoinType::InnerJoin,
                Alias::new("media"),
                Expr::col((Alias::new("media"), Alias::new("genre")))
                    .eq(user_media_view.genre())
                    .and(
                        Expr::col((Alias::new("media"), Alias::new("id")))
                            .equals((Alias::new("user_media_views"), Alias::new("media_id"))),
                    ),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
            .map_err(Into::into)
    }

    async fn get_by_media_type<'s>(
        &mut self,
        user_media_view: GetUserMediaViewByMediaType<'s>,
    ) -> Result<Vec<UserMediaView>, RepoError> {
        let (sql, values) = Query::select()
            .columns([
                (Alias::new("user_media_views"), Alias::new("id")),
                (Alias::new("user_media_views"), Alias::new("user_id")),
                (Alias::new("user_media_views"), Alias::new("media_id")),
                (Alias::new("user_media_views"), Alias::new("created")),
            ])
            .from(Alias::new("user_media_views"))
            .join(
                JoinType::InnerJoin,
                Alias::new("media"),
                Expr::col((Alias::new("media"), Alias::new("media_type")))
                    .eq(user_media_view.media_type())
                    .and(
                        Expr::col((Alias::new("media"), Alias::new("id")))
                            .equals((Alias::new("user_media_views"), Alias::new("media_id"))),
                    ),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
            .map_err(Into::into)
    }

    async fn get_by_media_age_restriction(
        &mut self,
        user_media_view: GetUserMediaViewByMediaAgeRestriction,
    ) -> Result<Vec<UserMediaView>, RepoError> {
        let (sql, values) = Query::select()
            .columns([
                (Alias::new("user_media_views"), Alias::new("id")),
                (Alias::new("user_media_views"), Alias::new("user_id")),
                (Alias::new("user_media_views"), Alias::new("media_id")),
                (Alias::new("user_media_views"), Alias::new("created")),
            ])
            .from(Alias::new("user_media_views"))
            .join(
                JoinType::InnerJoin,
                Alias::new("media"),
                Expr::col((Alias::new("media"), Alias::new("is_sfw")))
                    .eq(user_media_view.is_sfw())
                    .and(
                        Expr::col((Alias::new("media"), Alias::new("id")))
                            .equals((Alias::new("user_media_views"), Alias::new("media_id"))),
                    ),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
            .map_err(Into::into)
    }

    async fn get_by_media_source_id<'s>(
        &mut self,
        user_media_view: GetUserMediaViewByMediaSourceId<'s>,
    ) -> Result<Vec<UserMediaView>, RepoError> {
        let (sql, values) = Query::select()
            .columns([
                (Alias::new("user_media_views"), Alias::new("id")),
                (Alias::new("user_media_views"), Alias::new("user_id")),
                (Alias::new("user_media_views"), Alias::new("media_id")),
                (Alias::new("user_media_views"), Alias::new("created")),
            ])
            .from(Alias::new("user_media_views"))
            .join(
                JoinType::InnerJoin,
                Alias::new("media"),
                Expr::col((Alias::new("media"), Alias::new("source_id")))
                    .eq(*user_media_view.source_id())
                    .and(
                        Expr::col((Alias::new("media"), Alias::new("id")))
                            .equals((Alias::new("user_media_views"), Alias::new("media_id"))),
                    ),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|user_media_view_models: Vec<UserMediaViewModel>| {
                user_media_view_models.into_iter().map(Into::into).collect()
            })
            .map_err(Into::into)
    }
}
