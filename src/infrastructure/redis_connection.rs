use std::sync::Arc;

use deadpool::managed::{Pool, PoolConfig, Object};
use redis::Client;
use once_cell::sync::Lazy;
use tokio::{runtime::Handle, task};
use super::redis_async_pool::RedisConnectionManager;

pub static  REDIS_POOL : Lazy<Arc<Pool<RedisConnectionManager>>> = Lazy::new(|| {
    task::block_in_place(move || {
        Handle::current().block_on(async {
            let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
            let client = Client::open(redis_url).expect("Failed to create Redis client");

            let pool_config = PoolConfig::default();
            let connection_pool: Pool<RedisConnectionManager> =
                Pool::builder(RedisConnectionManager::new(client, true, None))
                .config(pool_config)
                .max_size(5)
                .build()
                .expect("Failed to create Redis pool");
            Arc::new(connection_pool)
        })
    })
});

pub fn get_redis_pool() -> Arc<Pool<RedisConnectionManager>> {
    REDIS_POOL.clone()
}

pub async  fn get_redis_connection() -> Object<RedisConnectionManager> {
    get_redis_pool().get().await.expect("Failed to get connection")
}