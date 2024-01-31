#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSourceByNameAndUrl<'a> {
    name: &'a str,
    url: &'a str,
}

impl<'a> GetSourceByNameAndUrl<'a> {
    pub const fn new(name: &'a str, url: &'a str) -> Self {
        Self { name, url }
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn url(&self) -> &str {
        self.url
    }
}
