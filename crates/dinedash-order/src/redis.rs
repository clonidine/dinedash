use mobc::Pool;
use mobc_redis::RedisConnectionManager;

pub const SERVER_ADDR: &str = "redis://order-cache:6379";

pub type RedisPool = Pool<RedisConnectionManager>;
