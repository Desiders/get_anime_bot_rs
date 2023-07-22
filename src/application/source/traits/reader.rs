use crate::{
    application::source::dto::{GetSourceById, GetSourceByName},
    domain::source::entities::Source as SourceEntity,
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait SourceReader {
    type GetError;
    type GetByIdError;
    type GetByNameError;

    async fn get_by_id(&mut self, source: GetSourceById) -> Result<SourceEntity, Self::GetError>;

    async fn get_by_name(
        &mut self,
        source: GetSourceByName,
    ) -> Result<SourceEntity, Self::GetByNameError>;
}
