use crate::application::{
    common::exceptions::{RepoError, RepoKind},
    user::{
        dto::{CreateUser, UpdateUserLanguageCode, UpdateUserShowNsfw},
        exceptions::UserTgIdAlreadyExists,
    },
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait UserRepo {
    async fn create(&mut self, user: CreateUser) -> Result<(), RepoKind<UserTgIdAlreadyExists>>;

    async fn update_language_code(&mut self, user: UpdateUserLanguageCode)
        -> Result<(), RepoError>;

    async fn update_show_nsfw(&mut self, user: UpdateUserShowNsfw) -> Result<(), RepoError>;
}
