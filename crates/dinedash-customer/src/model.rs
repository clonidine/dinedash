use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    name: String,
    id: Uuid,
}

impl Customer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            id: Uuid::new_v4(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
