use serde::{Deserialize, Serialize};

pub mod logon;
pub mod collection;
pub mod file_entity;

#[derive(Debug, Serialize)]
pub struct ApiResult {
    pub error_code: u32,
    pub message: String,
}

#[derive(Deserialize)]
pub struct PageQueryArgs {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub keyword: Option<String>,
}