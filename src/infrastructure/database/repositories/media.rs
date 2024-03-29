use crate::{
    application::{
        common::exceptions::{RepoError, RepoKind},
        media::{
            dto::{
                CreateMedia, GetMediaById, GetMediaByInfo, GetMediaByInfoUnviewedByUser,
                GetMediaByUrl,
            },
            exceptions::{MediaIdNotExist, MediaUrlAndGenreAlreadyExists},
            traits::{MediaReader, MediaRepo},
        },
    },
    domain::media::entities::{GenresStats, Media, MediaStats},
    infrastructure::database::models::{
        GenreStats as GenreStatsModel, Media as MediaModel, MediaStats as MediaStatsModel,
    },
};

use async_trait::async_trait;
use sea_query::{Alias, Expr, Func, JoinType, Order, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder as _;
use sqlx::PgConnection;

#[allow(clippy::module_name_repetitions)]
pub struct MediaRepoImpl<Conn> {
    conn: Conn,
}

impl<Conn> MediaRepoImpl<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> MediaRepo for MediaRepoImpl<&'a mut PgConnection> {
    async fn create<'s>(
        &mut self,
        media: CreateMedia<'s>,
    ) -> Result<(), RepoKind<MediaUrlAndGenreAlreadyExists>> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("media"))
            .columns([
                Alias::new("id"),
                Alias::new("url"),
                Alias::new("genre"),
                Alias::new("media_type"),
                Alias::new("is_sfw"),
                Alias::new("source_id"),
            ])
            .values_panic([
                (*media.id()).into(),
                media.url().into(),
                media.genre().into(),
                media.media_type().into(),
                media.is_sfw().into(),
                (*media.source_id()).into(),
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
                            return RepoKind::exception(MediaUrlAndGenreAlreadyExists::new(
                                media.url().to_string(),
                                media.genre().map(ToOwned::to_owned),
                                err.to_string(),
                            ));
                        }
                    }
                }
                RepoKind::unexpected(err)
            })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct MediaReaderImpl<Conn> {
    conn: Conn,
}

impl<Conn> MediaReaderImpl<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> MediaReader for MediaReaderImpl<&'a mut PgConnection> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_id<'s>(
        &mut self,
        media: GetMediaById<'s>,
    ) -> Result<Media, RepoKind<MediaIdNotExist>> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("url"),
                Alias::new("genre"),
                Alias::new("media_type"),
                Alias::new("is_sfw"),
                Alias::new("source_id"),
                Alias::new("created"),
            ])
            .from(Alias::new("media"))
            .and_where(Expr::col(Alias::new("id")).eq(*media.id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|media_model: MediaModel| media_model.into())
            .map_err(|err| {
                if let sqlx::Error::RowNotFound = err {
                    RepoKind::exception(MediaIdNotExist::new(*media.id(), err.to_string()))
                } else {
                    RepoKind::unexpected(err)
                }
            })
    }

    async fn get_by_url<'s>(&mut self, media: GetMediaByUrl<'s>) -> Result<Vec<Media>, RepoError> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("url"),
                Alias::new("genre"),
                Alias::new("media_type"),
                Alias::new("is_sfw"),
                Alias::new("source_id"),
                Alias::new("created"),
            ])
            .from(Alias::new("media"))
            .and_where(Expr::col(Alias::new("url")).eq(media.url()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|media_models: Vec<MediaModel>| media_models.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    async fn get_by_info<'s>(
        &mut self,
        media: GetMediaByInfo<'s>,
    ) -> Result<Vec<Media>, RepoError> {
        let mut query = Query::select();

        query
            .columns([
                Alias::new("id"),
                Alias::new("url"),
                Alias::new("genre"),
                Alias::new("media_type"),
                Alias::new("is_sfw"),
                Alias::new("source_id"),
                Alias::new("created"),
            ])
            .from(Alias::new("media"))
            .and_where(Expr::col(Alias::new("genre")).eq(media.genre()))
            .and_where(Expr::col(Alias::new("media_type")).eq(media.media_type()))
            .and_where(Expr::col(Alias::new("is_sfw")).eq(media.is_sfw()));

        if let Some(offset) = media.offset() {
            query.offset(offset);
        }
        if let Some(limit) = media.limit() {
            query.limit(limit);
        }

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|media_models: Vec<MediaModel>| media_models.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    async fn get_by_info_unviewed_by_user<'s>(
        &mut self,
        media: GetMediaByInfoUnviewedByUser<'s>,
    ) -> Result<Vec<Media>, RepoError> {
        let mut query = Query::select();

        query
            .columns([
                (Alias::new("media"), Alias::new("id")),
                (Alias::new("media"), Alias::new("url")),
                (Alias::new("media"), Alias::new("genre")),
                (Alias::new("media"), Alias::new("media_type")),
                (Alias::new("media"), Alias::new("is_sfw")),
                (Alias::new("media"), Alias::new("source_id")),
                (Alias::new("media"), Alias::new("created")),
            ])
            .from(Alias::new("media"))
            .join(
                JoinType::LeftJoin,
                Alias::new("user_media_views"),
                Expr::col((Alias::new("user_media_views"), Alias::new("user_id")))
                    .eq(*media.user_id())
                    .and(
                        Expr::col((Alias::new("user_media_views"), Alias::new("media_id")))
                            .equals((Alias::new("media"), Alias::new("id"))),
                    ),
            )
            .and_where(Expr::col((Alias::new("user_media_views"), Alias::new("id"))).is_null())
            .and_where(Expr::col((Alias::new("media"), Alias::new("genre"))).eq(media.genre()))
            .and_where(
                Expr::col((Alias::new("media"), Alias::new("media_type"))).eq(media.media_type()),
            )
            .and_where(Expr::col((Alias::new("media"), Alias::new("is_sfw"))).eq(media.is_sfw()))
            .order_by_expr(Func::random().into(), Order::Asc);

        if let Some(offset) = media.offset() {
            query.offset(offset);
        }
        if let Some(limit) = media.limit() {
            query.limit(limit);
        }

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|media_models: Vec<MediaModel>| media_models.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    async fn get_media_stats(&mut self) -> Result<MediaStats, RepoError> {
        let mut query = Query::select();

        query
            .expr_as(
                Func::count(Expr::col(Alias::new("id"))),
                Alias::new("total"),
            )
            .expr_as(
                Func::count(Expr::case(Expr::col(Alias::new("media_type")).eq("gif"), 1)),
                Alias::new("gif"),
            )
            .expr_as(
                Func::count(Expr::case(Expr::col(Alias::new("media_type")).eq("img"), 1)),
                Alias::new("image"),
            )
            .expr_as(
                Func::count(Expr::case(
                    Expr::col(Alias::new("media_type")).eq("unknown"),
                    1,
                )),
                Alias::new("unknown"),
            )
            .expr_as(
                Func::count(Expr::case(Expr::col(Alias::new("is_sfw")).eq(true), 1)),
                Alias::new("sfw"),
            )
            .expr_as(
                Func::count(Expr::case(Expr::col(Alias::new("is_sfw")).eq(false), 1)),
                Alias::new("nsfw"),
            )
            .from(Alias::new("media"));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|media_model: MediaStatsModel| media_model.into())
            .map_err(Into::into)
    }

    async fn get_genre_stats(&mut self) -> Result<GenresStats, RepoError> {
        let mut query = Query::select();

        query
            .expr_as(
                Func::count(Expr::col(Alias::new("id"))),
                Alias::new("total"),
            )
            .columns([
                Alias::new("genre"),
                Alias::new("media_type"),
                Alias::new("is_sfw"),
            ])
            .from(Alias::new("media"))
            .add_group_by([
                Expr::col(Alias::new("genre")).into(),
                Expr::col(Alias::new("media_type")).into(),
                Expr::col(Alias::new("is_sfw")).into(),
            ]);

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *self.conn)
            .await
            .map(|genre_stats_models: Vec<GenreStatsModel>| {
                genre_stats_models
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>()
                    .into()
            })
            .map_err(Into::into)
    }
}
