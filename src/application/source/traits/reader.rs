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
    async fn get_by_id<'s>(
        &mut self,
        source: GetSourceById<'s>,
    ) -> Result<SourceEntity, RepoKind<SourceIdNotExist>>;

    async fn get_by_name<'s>(
        &mut self,
        source: GetSourceByName<'s>,
    ) -> Result<Vec<SourceEntity>, RepoError>;

    async fn get_by_name_and_url<'s>(
        &mut self,
        source: GetSourceByNameAndUrl<'s>,
    ) -> Result<SourceEntity, RepoKind<SourceNameAndUrlNotExist>>;
}
