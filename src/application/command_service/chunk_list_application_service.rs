use chrono::Local;
use sea_orm::ActiveValue::Set;

use crate::domain::{command::file_command::AddChunkListCommand, model::entity::chunk_list, repository::chunk_list_repository};

/// 添加chunk
pub async fn add_chunk_list(command: AddChunkListCommand) -> Result<(), anyhow::Error> {
    let chunk = chunk_list_repository::get_chunk(&command.file_hash, command.chunk_number).await;
    if chunk.is_none() {
        let chunk_list_entity = chunk_list::ActiveModel {
            file_hash: Set(command.file_hash),
            chunk_number: Set(command.chunk_number),
            chunk_size: Set(command.chunk_size),
            file_name: Set(command.file_name),
            total_chunks: Set(command.total_chunks),
            created_time: Set(Local::now().naive_utc()),
            ..Default::default()
        };
        let _ = chunk_list_repository::add_chunk_list(chunk_list_entity).await?;
    }
    Ok(())
}