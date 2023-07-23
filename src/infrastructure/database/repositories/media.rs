use crate::{
    application::{
        common::exceptions::{RepoError, RepoKind},
        media::{
            dto::{CreateMedia, GetMediaById, GetMediaByUrl},
            exceptions::{MediaIdNotExist, MediaUrlAndGenreAlreadyExists},
            traits::{MediaReader, MediaRepo},
        },
    },
    domain::media::entities::Media,
    infrastructure::database::models::Media as MediaModel,
};

use async_trait::async_trait;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
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
    async fn create(
        &mut self,
        media: CreateMedia,
    ) -> Result<(), RepoKind<MediaUrlAndGenreAlreadyExists>> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("media"))
            .columns(vec![
                Alias::new("id"),
                Alias::new("url"),
                Alias::new("genre"),
                Alias::new("media_type"),
                Alias::new("is_sfw"),
                Alias::new("source_id"),
            ])
            .values_panic([
                media.id().into(),
                media.url().into(),
                media.genre().into(),
                media.media_type().into(),
                media.is_sfw().into(),
                media.source_id().into(),
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
    async fn get_by_id(&mut self, media: GetMediaById) -> Result<Media, RepoKind<MediaIdNotExist>> {
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
            .and_where(Expr::col(Alias::new("id")).eq(media.id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|media_model: MediaModel| media_model.into())
            .map_err(|err| {
                if let sqlx::Error::RowNotFound = err {
                    RepoKind::exception(MediaIdNotExist::new(media.id(), err.to_string()))
                } else {
                    RepoKind::unexpected(err)
                }
            })
    }

    async fn get_by_url(&mut self, media: GetMediaByUrl) -> Result<Vec<Media>, RepoError> {
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
}
