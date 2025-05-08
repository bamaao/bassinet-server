use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileEntityDTO {
    pub id: String,
    pub name: String,
    pub mime: String,
    pub description: Option<String>,
    pub path: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MultiFileEntityDTO {
    pub files: Vec<FileEntityDTO>,
}