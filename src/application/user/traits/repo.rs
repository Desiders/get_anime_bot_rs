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
    async fn create<'s>(
        &mut self,
        user: CreateUser<'s>,
    ) -> Result<(), RepoKind<UserTgIdAlreadyExists>>;

    async fn update_language_code<'s>(
        &mut self,
        user: UpdateUserLanguageCode<'s>,
    ) -> Result<(), RepoError>;

    async fn update_show_nsfw<'s>(&mut self, user: UpdateUserShowNsfw<'s>)
        -> Result<(), RepoError>;
}
