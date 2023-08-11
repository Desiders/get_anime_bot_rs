use super::repositories::{
    MediaReaderImpl, MediaRepoImpl, SourceReaderImpl, SourceRepoImpl, UserMediaViewReaderImpl,
    UserMediaViewRepoImpl, UserReaderImpl, UserRepoImpl,
};
use crate::application::{
    common::{
        exceptions::{BeginError, CommitError, RollbackError},
        traits::UnitOfWork,
    },
    media::traits::{MediaReader, MediaRepo},
    source::traits::{SourceReader, SourceRepo},
    user::traits::{UserReader, UserRepo},
    user_media_view::traits::{UserMediaViewReader, UserMediaViewRepo},
};

use async_trait::async_trait;
use sqlx::{pool::PoolConnection, Database};

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

pub struct SqlxUnitOfWork<DB>
where
    DB: Database,
{
    conn: PoolConnection<DB>,
}

impl<DB> SqlxUnitOfWork<DB>
where
    DB: Database,
{
    pub fn new(conn: PoolConnection<DB>) -> Self {
        Self { conn }
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

    fn connection(&mut self) -> Self::Connection<'_> {
        &mut self.conn
    }

    // async fn begin(&'static mut self) -> Result<Self::Connection<'_>, BeginError> {
    //     self.transaction = Some(self.conn.begin().await?);

    //     Ok(self.transaction.as_mut().unwrap())
    // }

    // async fn commit(&mut self) -> Result<(), CommitError> {
    //     if let Some(transaction) = self.transaction.take() {
    //         transaction.commit().await.map_err(Into::into)
    //     } else {
    //         Ok(())
    //     }
    // }

    // async fn rollback(&mut self) -> Result<(), RollbackError> {
    //     if let Some(transaction) = self.transaction.take() {
    //         transaction.rollback().await.map_err(Into::into)
    //     } else {
    //         Ok(())
    //     }
    // }

    fn user_repo(&mut self) -> Box<dyn UserRepo + Send + '_> {
        Box::new(UserRepoImpl::new(self.connection()))
    }

    fn user_reader(&mut self) -> Box<dyn UserReader + Send + '_> {
        Box::new(UserReaderImpl::new(self.connection()))
    }

    fn source_repo(&mut self) -> Box<dyn SourceRepo + Send + '_> {
        Box::new(SourceRepoImpl::new(self.connection()))
    }

    fn source_reader(&mut self) -> Box<dyn SourceReader + Send + '_> {
        Box::new(SourceReaderImpl::new(self.connection()))
    }

    fn media_repo(&mut self) -> Box<dyn MediaRepo + Send + '_> {
        Box::new(MediaRepoImpl::new(self.connection()))
    }

    fn media_reader(&mut self) -> Box<dyn MediaReader + Send + '_> {
        Box::new(MediaReaderImpl::new(self.connection()))
    }

    fn user_media_view_repo(&mut self) -> Box<dyn UserMediaViewRepo + Send + '_> {
        Box::new(UserMediaViewRepoImpl::new(self.connection()))
    }

    fn user_media_view_reader(&mut self) -> Box<dyn UserMediaViewReader + Send + '_> {
        Box::new(UserMediaViewReaderImpl::new(self.connection()))
    }
}
