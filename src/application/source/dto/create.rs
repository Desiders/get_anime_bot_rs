use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSource<'a> {
    id: &'a Uuid,
    name: &'a str,
    url: &'a str,
}

impl<'a> CreateSource<'a> {
    pub const fn new(id: &'a Uuid, name: &'a str, url: &'a str) -> Self {
        Self { id, name, url }
    }

    pub const fn id(&self) -> &Uuid {
        self.id
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn url(&self) -> &str {
        self.url
    }
}
