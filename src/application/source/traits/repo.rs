use crate::application::{
    common::exceptions::RepoKind,
    source::{dto::CreateSource, exceptions::SourceNameAndUrlAlreadyExists},
};

use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
#[async_trait]
pub trait SourceRepo {
    async fn create(
        &mut self,
        source: CreateSource,
    ) -> Result<(), RepoKind<SourceNameAndUrlAlreadyExists>>;
}
