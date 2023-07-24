use crate::application::media_parser::traits::Source;

use async_trait::async_trait;
use std::sync::Arc;
use telers::{
    error::EventErrorKind,
    event::telegram::{HandlerRequest, HandlerResponse},
    middlewares::{InnerMiddleware, Next},
};

#[derive(Default)]
pub struct MediaParserSources {
    sources: Vec<Arc<dyn Source>>,
}

impl MediaParserSources {
    pub fn new<T, S, I>(sources: I) -> Self
    where
        S: Source + 'static,
        T: Into<Arc<S>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            sources: sources
                .into_iter()
                .map(|source| source.into() as _)
                .collect(),
        }
    }

    pub fn source<S>(self, source: impl Into<Arc<S>>) -> Self
    where
        S: Source + 'static,
    {
        Self {
            sources: self
                .sources
                .into_iter()
                .chain(Some(source.into() as _))
                .collect(),
        }
    }

    pub fn sources<T, S, I>(self, sources: I) -> Self
    where
        S: Source + 'static,
        T: Into<Arc<S>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            sources: self
                .sources
                .into_iter()
                .chain(sources.into_iter().map(|source| source.into() as _))
                .collect(),
        }
    }
}

#[async_trait]
impl<Client> InnerMiddleware<Client> for MediaParserSources
where
    Client: Send + Sync + 'static,
{
    async fn call(
        &self,
        request: HandlerRequest<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, EventErrorKind> {
        request
            .context
            .insert("media_parser_sources", Box::new(self.sources.clone()));

        next(request).await
    }
}
