use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    id: Uuid,
    customer_id: Uuid,
    cart_id: Uuid,
    status: OrderStatus,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum OrderStatus {
    Pendent,
    Cancelled,
    Approved,
}

impl Order {
    pub fn new(customer_id: Uuid, cart_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            customer_id,
            cart_id,
            status: OrderStatus::Pendent,
            description: None,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn with_description(self, description: &str) -> Self {
        Self {
            description: Some(description.to_owned()),
            ..self
        }
    }

    pub fn with_status(self, status: OrderStatus) -> Self {
        Self { status, ..self }
    }
}
