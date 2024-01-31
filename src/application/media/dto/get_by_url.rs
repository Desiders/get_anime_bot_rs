#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMediaByUrl<'a> {
    url: &'a str,
}

impl<'a> GetMediaByUrl<'a> {
    pub const fn new(url: &'a str) -> Self {
        Self { url }
    }

    pub const fn url(&self) -> &str {
        self.url
    }
}
