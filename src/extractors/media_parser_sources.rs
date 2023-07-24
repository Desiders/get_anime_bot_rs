use crate::application::media_parser::traits::Source;

use std::sync::Arc;
use telers::{
    client::Bot, context::Context, error::ExtractionError, extract::FromEventAndContext,
    types::Update,
};

/// Wrapper for foregein [`Vec<Arc<dyn Source>>`] to be used in [`FromEventAndContext`] extractor
pub struct MediaParserSourceWrapper {
    inner: Vec<Arc<dyn Source>>,
}

impl MediaParserSourceWrapper {
    pub fn new(inner: Vec<Arc<dyn Source>>) -> Self {
        Self { inner }
    }

    /// Returns inner [`Vec<Arc<dyn Source>>`] wrapped in [`Arc`]
    pub fn inner(&self) -> Vec<Arc<dyn Source>> {
        self.inner.clone()
    }
}

impl<Client> FromEventAndContext<Client> for MediaParserSourceWrapper {
    type Error = ExtractionError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        let Some(result) = context.get("media_parser_sources") else {
            return Err(ExtractionError::new("No sources found in context"));
        };

        let sources = if let Some(sources) = result.downcast_ref::<Vec<Arc<dyn Source>>>() {
            sources.clone()
        } else {
            return Err(ExtractionError::new(
                "Sources in context is not a correct Vec<Arc<dyn Source>>",
            ));
        };

        Ok(Self::new(sources))
    }
}
