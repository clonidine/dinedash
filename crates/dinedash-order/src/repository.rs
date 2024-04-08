use uuid::Uuid;

use crate::{model::Order, redis::RedisPool};

pub async fn get(_pool: &RedisPool) -> Vec<Order> {
    todo!()
}

pub async fn find(_order_id: Uuid) -> Order {
    todo!()
}

pub async fn delete(_order_id: Uuid) -> anyhow::Result<u64> {
    todo!()
}

pub async fn insert(_order: &Order) -> anyhow::Result<u64> {
    todo!()
}
