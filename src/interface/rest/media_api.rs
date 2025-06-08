
// use axum::{extract::Path, http::StatusCode, response::IntoResponse};

// use crate::{application::{query_service::{account_query_service, media_query_service}}, domain::{repository::{collection_repository, media_repository}}, infrastructure::jwt::Claims};

// /// 获取ViewingKey
// /// 是否可查看: 
// /// 1. 视频所属专辑是公开的且视频是公开的
// /// 2. 视频所属专辑非公开但已经Minting相应NFT
// pub async fn get_viewing_key(claims: Claims, Path((collection_id, media_id)): Path<(String,String)>)-> impl IntoResponse {
//     let pubkey = claims.pubkey;
//     let account = account_query_service::get_account_info(&pubkey).await;
//     if account.is_err() {
//         return (StatusCode::INTERNAL_SERVER_ERROR, "未知账号".to_owned())
//     }
//     let account_info = account.unwrap();
//     let collection = collection_repository::get_by_id(&collection_id).await;
//     if collection.is_none() {
//         return (StatusCode::INTERNAL_SERVER_ERROR, "未知专辑".to_owned())
//     }
//     let media = media_repository::get_by_id(&media_id, &collection_id).await;
//     if media.is_none() {
//         return (StatusCode::INTERNAL_SERVER_ERROR, "未知视频".to_owned())
//     }

//     let collection = collection.unwrap();
//     let viewing_key = media_query_service::viewing_key(account_info.account_id, account_info.wallet_address, &collection).await;
//     if viewing_key.is_some() {
//         return (StatusCode::OK, viewing_key.unwrap())
//     }
//     (StatusCode::INTERNAL_SERVER_ERROR, "未知视频".to_owned())
// }