#[derive(Debug)]
pub struct AddFileCommand {
    pub mime: String,
    pub file_name: String,
    pub description: Option<String>,
    // pub length: u64,
    pub path: String,
    pub hash: Option<String>
}