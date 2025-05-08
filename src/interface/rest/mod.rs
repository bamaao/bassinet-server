use axum::{http::StatusCode, response::IntoResponse};
use redis::{AsyncCommands, RedisError};
use uuid::Uuid;

use crate::infrastructure::redis_connection;

pub mod dto;
pub mod assembler;
pub mod my_collection_api;
pub mod public_collection_api;
pub mod logon_api;
pub mod account;
pub mod validate;
pub mod file_api;

pub async fn request_id() -> impl IntoResponse {
    // let pool = redis_connection::get_redis_pool();
    // let mut connection = pool.get().await.expect("Failed to get connection");
    let mut connection = redis_connection::get_redis_connection().await;
    let uuid = Uuid::new_v4().to_string();

    let _ : Result<(), RedisError>= connection
    .set_ex(&uuid, 1 as u64, 60)
    .await;
    
    (StatusCode::OK, uuid)
}