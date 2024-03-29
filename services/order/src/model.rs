use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Order<'a> {
    id: Uuid,
    cart_id: Uuid,
    status: OrderStatus,
    description: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderStatus {
    Pendent,
    Cancelled,
    Approved,
}

impl<'a> Order<'a> {
    pub fn new(cart_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            cart_id,
            status: OrderStatus::Pendent,
            description: None,
        }
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);

        self
    }

    pub fn status(mut self, status: OrderStatus) -> Self {
        self.status = status;

        self
    }
}
