#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMediaByUrl {
    url: String,
}

impl GetMediaByUrl {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
