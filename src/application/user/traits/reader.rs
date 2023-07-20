use crate::{
    application::user::dto::{GetUserById, GetUserByTgId},
    domain::user::entities::User as UserEntity,
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait UserReader {
    type GetError;
    type GetByIdError;

    async fn get_by_id(&mut self, user: GetUserById) -> Result<UserEntity, Self::GetError>;

    async fn get_by_tg_id(&mut self, user: GetUserByTgId)
        -> Result<UserEntity, Self::GetByIdError>;
}
