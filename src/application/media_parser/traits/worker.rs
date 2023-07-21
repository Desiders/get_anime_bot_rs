use super::source::Source;
use crate::domain::media_parser::entities::Media;

use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

#[async_trait]
pub trait Worker<S>
where
    S: Source,
{
    async fn parse(self, source: S) -> Receiver<Media>;
}
