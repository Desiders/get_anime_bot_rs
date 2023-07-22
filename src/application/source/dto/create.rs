use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSource {
    id: Uuid,
    name: String,
    url: String,
}

impl CreateSource {
    pub fn new(id: Uuid, name: String, url: String) -> Self {
        Self { id, name, url }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
