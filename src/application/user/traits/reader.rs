use crate::application::user::dto::{GetUserById, GetUserByTgId, User};

use async_trait::async_trait;

#[async_trait]
pub trait UserReader {
    type GetError;
    type GetByIdError;

    async fn get_by_id(&mut self, user: GetUserById) -> Result<User, Self::GetError>;

    async fn get_by_tg_id(&mut self, user: GetUserByTgId) -> Result<User, Self::GetByIdError>;
}
