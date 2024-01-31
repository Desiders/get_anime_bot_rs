use crate::{
    application::{
        common::exceptions::{RepoError, RepoKind},
        source::{
            dto::{CreateSource, GetSourceById, GetSourceByName, GetSourceByNameAndUrl},
            exceptions::{
                SourceIdNotExist, SourceNameAndUrlAlreadyExists, SourceNameAndUrlNotExist,
            },
            traits::{SourceReader, SourceRepo},
        },
    },
    domain::source::entities::Source,
    infrastructure::database::models::Source as SourceModel,
};

use async_trait::async_trait;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder as _;
use sqlx::PgConnection;

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
    async fn create<'s>(
        &mut self,
        source: CreateSource<'s>,
    ) -> Result<(), RepoKind<SourceNameAndUrlAlreadyExists>> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("sources"))
            .columns([Alias::new("id"), Alias::new("name"), Alias::new("url")])
            .values_panic([
                (*source.id()).into(),
                source.name().into(),
                source.url().into(),
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
                            return RepoKind::exception(SourceNameAndUrlAlreadyExists::new(
                                source.name().to_string(),
                                source.url().to_string(),
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
    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_id<'s>(
        &mut self,
        source: GetSourceById<'s>,
    ) -> Result<Source, RepoKind<SourceIdNotExist>> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("name"),
                Alias::new("url"),
                Alias::new("created"),
            ])
            .from(Alias::new("sources"))
            .and_where(Expr::col(Alias::new("id")).eq(*source.id()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|source_model: SourceModel| source_model.into())
            .map_err(|err| {
                if let sqlx::Error::RowNotFound = err {
                    RepoKind::exception(SourceIdNotExist::new(*source.id(), err.to_string()))
                } else {
                    RepoKind::unexpected(err)
                }
            })
    }

    async fn get_by_name<'s>(
        &mut self,
        source: GetSourceByName<'s>,
    ) -> Result<Vec<Source>, RepoError> {
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
            .fetch_all(&mut *self.conn)
            .await
            .map(|source_models: Vec<SourceModel>| {
                source_models.into_iter().map(Into::into).collect()
            })
            .map_err(Into::into)
    }

    #[allow(clippy::redundant_closure_for_method_calls)]
    async fn get_by_name_and_url<'s>(
        &mut self,
        source: GetSourceByNameAndUrl<'s>,
    ) -> Result<Source, RepoKind<SourceNameAndUrlNotExist>> {
        let (sql, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("name"),
                Alias::new("url"),
                Alias::new("created"),
            ])
            .from(Alias::new("sources"))
            .and_where(Expr::col(Alias::new("name")).eq(source.name()))
            .and_where(Expr::col(Alias::new("url")).eq(source.url()))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *self.conn)
            .await
            .map(|source_model: SourceModel| source_model.into())
            .map_err(|err| {
                if let sqlx::Error::RowNotFound = err {
                    RepoKind::exception(SourceNameAndUrlNotExist::new(
                        source.name().to_owned(),
                        source.url().to_owned(),
                        err.to_string(),
                    ))
                } else {
                    RepoKind::unexpected(err)
                }
            })
    }
}
