use crate::{
    application::{
        common::exceptions::RepoKind,
        user::{
            dto::{GetUserById, GetUserByTgId},
            exceptions::{UserIdNotExist, UserTgIdNotExist},
        },
    },
    domain::user::entities::User as UserEntity,
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait UserReader {
    async fn get_by_id<'s>(
        &mut self,
        user: GetUserById<'s>,
    ) -> Result<UserEntity, RepoKind<UserIdNotExist>>;

    async fn get_by_tg_id(
        &mut self,
        user: GetUserByTgId,
    ) -> Result<UserEntity, RepoKind<UserTgIdNotExist>>;
}
