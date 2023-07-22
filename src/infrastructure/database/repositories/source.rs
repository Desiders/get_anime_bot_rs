use crate::{
    application::source::{
        dto::{CreateSource, GetSourceById, GetSourceByName},
        traits::{SourceReader, SourceRepo},
    },
    domain::source::entities::Source,
    infrastructure::database::models::Source as SourceModel,
};

use async_trait::async_trait;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder as _;
use sqlx::{Error, PgConnection};

#[allow(clippy::module_name_repetitions)]
pub struct SourceRepoImpl<Conn> {
    conn: Conn,
}

impl<Conn> SourceRepoImpl<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> SourceRepo for SourceRepoImpl<&'a mut PgConnection> {
    type CreateError = Error;

    async fn create(&mut self, source: CreateSource) -> Result<(), Self::CreateError> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("sources"))
            .columns(vec![
                Alias::new("id"),
                Alias::new("name"),
                Alias::new("url"),
            ])
            .values_panic([
                source.id().into(),
                source.name().into(),
                source.url().into(),
            ])
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut *self.conn)
            .await
            .map(|_| ())
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct SourceReaderImpl<Conn> {
    conn: Conn,
}

impl<Conn> SourceReaderImpl<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> SourceReader for SourceReaderImpl<&'a mut PgConnection> {
    type GetError = Error;
    type GetByIdError = Error;
    type GetByNameError = Error;

    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_id(&mut self, user: GetSourceById) -> Result<Source, Self::GetError> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("name"),
                Alias::new("url"),
                Alias::new("created"),
            ])
            .from(Alias::new("sources"))
            .and_where(Expr::col(Alias::new("id")).eq(user.id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|source_model: SourceModel| source_model.into())
    }

    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_name(
        &mut self,
        source: GetSourceByName,
    ) -> Result<Source, Self::GetByNameError> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("name"),
                Alias::new("url"),
                Alias::new("created"),
            ])
            .from(Alias::new("sources"))
            .and_where(Expr::col(Alias::new("name")).eq(source.name()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|source_model: SourceModel| source_model.into())
    }
}
