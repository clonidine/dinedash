use sqlx::types::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Customer {
    name: String,
    id: Uuid,
}

impl Customer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            id: Uuid::now_v7(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
