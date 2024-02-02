use crate::application::media_parser::traits::Source;

use async_trait::async_trait;
use std::sync::Arc;
use telers::{
    errors::EventErrorKind,
    event::telegram::{HandlerRequest, HandlerResponse},
    middlewares::{InnerMiddleware, Next},
};

#[derive(Default)]
pub struct MediaParserSources {
    sources: Vec<Arc<dyn Source>>,
}

impl MediaParserSources {
    pub fn source<S>(mut self, source: impl Into<Arc<S>>) -> Self
    where
        S: Source + 'static,
    {
        self.sources.push(source.into());
        self
    }
}

#[async_trait]
impl InnerMiddleware for MediaParserSources {
    async fn call(
        &self,
        request: HandlerRequest,
        next: Next,
    ) -> Result<HandlerResponse, EventErrorKind> {
        request
            .context
            .insert("media_parser_sources", Box::new(self.sources.clone()));

        next(request).await
    }
}
