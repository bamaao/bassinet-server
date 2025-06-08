use crate::{domain::repository::chunk_list_repository, interface::rest::dto::media::ChunkListDTO};

/// 获取指定md5的所有分片
pub async fn query_chunk_list(md5: &String) -> Vec<ChunkListDTO> {
    let chunks = chunk_list_repository::query_chunk_list(md5).await;
    if chunks.is_empty() {
        return Vec::new()
    }
    let mut results = Vec::new();
    for chunk in chunks {
        results.push(ChunkListDTO {
            file_hash: chunk.file_hash,
            chunk_number: chunk.chunk_number,
            chunk_size: chunk.chunk_size,
            file_name: chunk.file_name,
            total_chunks: chunk.total_chunks,
        });
    }
    results
}