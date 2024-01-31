#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSourceByName<'a> {
    name: &'a str,
}

impl<'a> GetSourceByName<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &str {
        self.name
    }
}
