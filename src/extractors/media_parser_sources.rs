use crate::application::media_parser::traits::Source;

use std::sync::Arc;
use telers::{
    client::Bot,
    context::Context,
    errors::ExtractionError,
    extractors::{from_context_into, FromEventAndContext},
    types::Update,
};

pub struct MediaParserSourceWrapper(pub Vec<Arc<dyn Source>>);

impl MediaParserSourceWrapper {
    pub fn inner(&self) -> &[Arc<dyn Source>] {
        &self.0
    }
}

impl From<Vec<Arc<dyn Source>>> for MediaParserSourceWrapper {
    fn from(sources: Vec<Arc<dyn Source>>) -> Self {
        Self(sources)
    }
}

from_context_into!([Client], Vec<Arc<dyn Source>> => MediaParserSourceWrapper, "media_parser_sources");
