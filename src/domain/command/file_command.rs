#[derive(Debug)]
pub struct AddFileCommand {
    pub mime: String,
    pub file_name: String,
    pub description: Option<String>,
    // pub length: u64,
    pub path: String,
    pub hash: Option<String>
}

#[derive(Debug)]
pub struct AddChunkListCommand {
    pub file_hash: String,
    pub chunk_number: i32,
    pub chunk_size: i32,
    pub file_name: String,
    pub total_chunks: i32
}