use serde::{Deserialize, Serialize};

/// 文件分片
#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkListDTO {
    pub file_hash: String,
    pub chunk_number: i32,
    pub chunk_size: i32,
    pub file_name: String,
    pub total_chunks: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkInfoDTO {
    pub upload_status: u8,
    pub chunk_sign_arr: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaDTO {
    pub file_hash: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddVideoPayload {
    pub request_id: String,
    pub title: String,
    pub description: String,
    pub is_public: u32,
    pub video_path: String,
    pub collection_id: String,
    pub file_hash: String
}