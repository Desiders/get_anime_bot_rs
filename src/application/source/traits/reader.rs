use crate::{
    application::{
        common::exceptions::{RepoError, RepoKind},
        source::{
            dto::{GetSourceById, GetSourceByName, GetSourceByNameAndUrl},
            exceptions::{SourceIdNotExist, SourceNameAndUrlNotExist},
        },
    },
    domain::source::entities::Source as SourceEntity,
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait SourceReader {
    async fn get_by_id(
        &mut self,
        source: GetSourceById,
    ) -> Result<SourceEntity, RepoKind<SourceIdNotExist>>;

    async fn get_by_name(
        &mut self,
        source: GetSourceByName,
    ) -> Result<Vec<SourceEntity>, RepoError>;

    async fn get_by_name_and_url(
        &mut self,
        source: GetSourceByNameAndUrl,
    ) -> Result<SourceEntity, RepoKind<SourceNameAndUrlNotExist>>;
}
