// 修改头像、修改昵称

use axum::{http::StatusCode, Json};

use crate::{application::query_service::account_query_service, infrastructure::jwt::Claims};

use super::dto::account::AccountInfo;

/// 获取用户账户信息
pub async fn get_account_info(claims: Claims) -> Result<Json<AccountInfo>, (StatusCode, String)> {
    //map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
    let result = account_query_service::get_account_info(&claims.pubkey).await;
    if result.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, result.err().unwrap().to_string()))
    }
    Ok(Json(result.unwrap()))
}

pub async fn get_authors() -> Result<Json<Vec<AccountInfo>>, (StatusCode, String)> {
    let authors = account_query_service::get_authors().await;
    if authors.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, authors.err().unwrap().to_string()))
    }
    Ok(Json(authors.unwrap()))
}
