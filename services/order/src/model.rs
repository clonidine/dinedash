use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    id: Uuid,
    cart_id: Uuid,
    status: OrderStatus,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum OrderStatus {
    Pendent,
    Cancelled,
    Approved,
}

impl Order {
    pub fn new(cart_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            cart_id,
            status: OrderStatus::Pendent,
            description: None,
        }
    }

    pub fn description(&self, description: &str) -> Self {
        Self {
            description: Some(description.to_string()),
            ..*self
        }
    }

    pub fn status(&mut self, status: OrderStatus) -> Self {
        Self {
            status,
            description: self.description.take(),
            ..*self
        }
    }
}
