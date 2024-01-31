use crate::application::media_parser::traits::Source;

use std::sync::Arc;
use telers::FromContext;

#[derive(FromContext)]
#[context(key = "media_parser_sources", from = Vec<Arc<dyn Source>>)]
pub struct MediaParserSourceWrapper(pub Vec<Arc<dyn Source>>);

impl From<Vec<Arc<dyn Source>>> for MediaParserSourceWrapper {
    fn from(sources: Vec<Arc<dyn Source>>) -> Self {
        Self(sources)
    }
}
