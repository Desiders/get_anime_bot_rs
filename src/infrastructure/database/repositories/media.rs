use crate::{
    application::media::{
        dto::{CreateMedia, GetMediaById, GetMediaByUrl},
        traits::{MediaReader, MediaRepo},
    },
    domain::media::entities::Media,
    infrastructure::database::models::Media as MediaModel,
};

use async_trait::async_trait;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder as _;
use sqlx::{Error, PgConnection};

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
    type CreateError = Error;

    async fn create(&mut self, media: CreateMedia) -> Result<(), Self::CreateError> {
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
    type GetError = Error;
    type GetByIdError = Error;

    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_id(&mut self, user: GetMediaById) -> Result<Media, Self::GetError> {
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
            .and_where(Expr::col(Alias::new("id")).eq(user.id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|media_model: MediaModel| media_model.into())
    }

    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_url(&mut self, media: GetMediaByUrl) -> Result<Media, Self::GetByIdError> {
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
            .fetch_one(&mut *self.conn)
            .await
            .map(|media_model: MediaModel| media_model.into())
    }
}
