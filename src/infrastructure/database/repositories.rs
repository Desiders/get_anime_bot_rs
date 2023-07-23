pub mod media;
pub mod source;
pub mod user;
pub mod user_media_view;

pub use media::{MediaReaderImpl, MediaRepoImpl};
pub use source::{SourceReaderImpl, SourceRepoImpl};
pub use user::{UserReaderImpl, UserRepoImpl};
pub use user_media_view::{UserMediaViewReaderImpl, UserMediaViewRepoImpl};

use crate::application::common::exceptions::{ApplicationException, RepoError, RepoKind};

impl From<sqlx::Error> for RepoError {
    fn from(error: sqlx::Error) -> Self {
        Self::new(error.to_string())
    }
}

impl<RepoException> From<sqlx::Error> for RepoKind<RepoException>
where
    RepoException: ApplicationException,
{
    fn from(error: sqlx::Error) -> Self {
        Self::unexpected(error)
    }
}
