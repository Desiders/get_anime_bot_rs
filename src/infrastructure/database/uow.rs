use super::repositories::{
    MediaReaderImpl, MediaRepoImpl, SourceReaderImpl, SourceRepoImpl, UserMediaViewReaderImpl,
    UserMediaViewRepoImpl, UserReaderImpl, UserRepoImpl,
};
use crate::application::{
    common::{
        exceptions::{BeginError, CommitError, RollbackError},
        traits::{UnitOfWork, UnitOfWorkFactory},
    },
    media::traits::{MediaReader, MediaRepo},
    source::traits::{SourceReader, SourceRepo},
    user::traits::{UserReader, UserRepo},
    user_media_view::traits::{UserMediaViewReader, UserMediaViewRepo},
};

use async_trait::async_trait;
use sqlx::{Database, Pool, Transaction};
use tracing::instrument;

impl From<sqlx::Error> for BeginError {
    fn from(error: sqlx::Error) -> Self {
        Self::new(error.to_string())
    }
}

impl From<sqlx::Error> for CommitError {
    fn from(error: sqlx::Error) -> Self {
        Self::new(error.to_string())
    }
}

impl From<sqlx::Error> for RollbackError {
    fn from(error: sqlx::Error) -> Self {
        Self::new(error.to_string())
    }
}

pub struct SqlxUnitOfWorkFactory<DB>
where
    DB: Database,
{
    pool: Pool<DB>,
}

impl<DB> SqlxUnitOfWorkFactory<DB>
where
    DB: Database,
{
    pub const fn new(pool: Pool<DB>) -> Self {
        Self { pool }
    }
}

impl<DB> Clone for SqlxUnitOfWorkFactory<DB>
where
    DB: Database,
{
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

impl<DB> UnitOfWorkFactory for SqlxUnitOfWorkFactory<DB>
where
    DB: Database,
    for<'a> UserRepoImpl<&'a mut DB::Connection>: UserRepo,
    for<'a> UserReaderImpl<&'a mut DB::Connection>: UserReader,
    for<'a> SourceRepoImpl<&'a mut DB::Connection>: SourceRepo,
    for<'a> SourceReaderImpl<&'a mut DB::Connection>: SourceReader,
    for<'a> MediaRepoImpl<&'a mut DB::Connection>: MediaRepo,
    for<'a> MediaReaderImpl<&'a mut DB::Connection>: MediaReader,
    for<'a> UserMediaViewRepoImpl<&'a mut DB::Connection>: UserMediaViewRepo,
    for<'a> UserMediaViewReaderImpl<&'a mut DB::Connection>: UserMediaViewReader,
{
    type UnitOfWork = SqlxUnitOfWork<DB>;

    fn new_unit_of_work(&self) -> Self::UnitOfWork {
        SqlxUnitOfWork::new(self.pool.clone())
    }
}

pub struct SqlxUnitOfWork<DB>
where
    DB: Database,
{
    pool: Pool<DB>,
    transaction: Option<Transaction<'static, DB>>,
}

impl<DB> SqlxUnitOfWork<DB>
where
    DB: Database,
{
    pub fn new(pool: Pool<DB>) -> Self {
        Self {
            pool,
            transaction: None,
        }
    }
}

#[async_trait]
impl<DB> UnitOfWork for SqlxUnitOfWork<DB>
where
    DB: Database,
    for<'a> UserRepoImpl<&'a mut DB::Connection>: UserRepo,
    for<'a> UserReaderImpl<&'a mut DB::Connection>: UserReader,
    for<'a> SourceRepoImpl<&'a mut DB::Connection>: SourceRepo,
    for<'a> SourceReaderImpl<&'a mut DB::Connection>: SourceReader,
    for<'a> MediaRepoImpl<&'a mut DB::Connection>: MediaRepo,
    for<'a> MediaReaderImpl<&'a mut DB::Connection>: MediaReader,
    for<'a> UserMediaViewRepoImpl<&'a mut DB::Connection>: UserMediaViewRepo,
    for<'a> UserMediaViewReaderImpl<&'a mut DB::Connection>: UserMediaViewReader,
{
    type Connection<'a> = &'a mut DB::Connection where Self: 'a;

    #[instrument(skip_all)]
    async fn connection(&mut self) -> Result<Self::Connection<'_>, BeginError> {
        if self.transaction.is_none() {
            self.begin().await?;
        }

        Ok(self.transaction.as_mut().unwrap())
    }

    #[instrument(skip_all)]
    async fn begin(&mut self) -> Result<(), BeginError> {
        match self.pool.try_begin().await? {
            Some(transaction) => {
                self.transaction = Some(transaction);
            }
            None => {
                self.transaction = Some(self.pool.begin().await?);
            }
        }

        Ok(())
    }

    #[instrument(skip_all)]
    async fn commit(&mut self) -> Result<(), CommitError> {
        if let Some(transaction) = self.transaction.take() {
            transaction.commit().await.map_err(Into::into)
        } else {
            Ok(())
        }
    }

    async fn rollback(&mut self) -> Result<(), RollbackError> {
        if let Some(transaction) = self.transaction.take() {
            transaction.rollback().await.map_err(Into::into)
        } else {
            Ok(())
        }
    }

    #[instrument(skip_all)]
    async fn user_repo(&mut self) -> Result<Box<dyn UserRepo + Send + '_>, BeginError> {
        Ok(Box::new(UserRepoImpl::new(self.connection().await?)))
    }

    #[instrument(skip_all)]
    async fn user_reader(&mut self) -> Result<Box<dyn UserReader + Send + '_>, BeginError> {
        Ok(Box::new(UserReaderImpl::new(self.connection().await?)))
    }

    #[instrument(skip_all)]
    async fn source_repo(&mut self) -> Result<Box<dyn SourceRepo + Send + '_>, BeginError> {
        Ok(Box::new(SourceRepoImpl::new(self.connection().await?)))
    }

    #[instrument(skip_all)]
    async fn source_reader(&mut self) -> Result<Box<dyn SourceReader + Send + '_>, BeginError> {
        Ok(Box::new(SourceReaderImpl::new(self.connection().await?)))
    }

    #[instrument(skip_all)]
    async fn media_repo(&mut self) -> Result<Box<dyn MediaRepo + Send + '_>, BeginError> {
        Ok(Box::new(MediaRepoImpl::new(self.connection().await?)))
    }

    #[instrument(skip_all)]
    async fn media_reader(&mut self) -> Result<Box<dyn MediaReader + Send + '_>, BeginError> {
        Ok(Box::new(MediaReaderImpl::new(self.connection().await?)))
    }

    #[instrument(skip_all)]
    async fn user_media_view_repo(
        &mut self,
    ) -> Result<Box<dyn UserMediaViewRepo + Send + '_>, BeginError> {
        Ok(Box::new(UserMediaViewRepoImpl::new(
            self.connection().await?,
        )))
    }

    #[instrument(skip_all)]
    async fn user_media_view_reader(
        &mut self,
    ) -> Result<Box<dyn UserMediaViewReader + Send + '_>, BeginError> {
        Ok(Box::new(UserMediaViewReaderImpl::new(
            self.connection().await?,
        )))
    }
}
