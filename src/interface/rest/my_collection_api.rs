use std::sync::Arc;

use crate::{application::query_service::my_collection_query_service, domain::{command::collection_command::AddVideoCommand, repository::account_repository}, infrastructure::jwt::Claims, ServerConfig};

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{application::command_service::collection_application_service, domain::command::collection_command::{CreateArticleCommand, CreateCollectionCommand}, interface::rest::validate::validate_request_id};

use super::dto::{collection::{ArticleDTO, CollectionDTO, CollectionInfoDTO, CollectionListDTO, CollectionPageDTOList, PageInfo}, media::AddVideoPayload, PageQueryArgs};


/// 创建专辑
pub async fn create_collection(State(config): State<Arc<ServerConfig>>, claims: Claims, Json(payload): Json<CollectionDTO>) -> impl IntoResponse {
    tracing::debug!("{}", serde_json::to_string(&claims).unwrap());
    tracing::debug!("{}", serde_json::to_string(&payload).unwrap());
    let validate_result = validate_request_id(&payload.request_id);
    if validate_result.is_err() {
        return validate_result.err().unwrap();
    }

    let file_path = std::path::Path::new(&config.assets_path).join("icons").join(&payload.icon_path);
    if !file_path.is_file() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "图片文件不存在".to_owned());
    }

    let command = CreateCollectionCommand {
        title: payload.title,
        description: payload.description,
        is_public: payload.is_public,
        pub_key: claims.pubkey,
        icon_path: payload.icon_path,
    };
    let application_result = collection_application_service::create_collection(command, &file_path, &config.assets_path).await;
    if application_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, application_result.err().unwrap().to_string());
    }
    (StatusCode::OK, application_result.unwrap())
}

/// 所有专辑(id,title),创建图文时使用
pub async fn get_simple_collections(claims: Claims) -> Result<Json<CollectionListDTO>, (StatusCode, String)> {
    let exist_accounts = account_repository::find_by_pubkey(&claims.pubkey).await;
    if exist_accounts.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "未知账户".to_owned()));
    }
    let result = my_collection_query_service::get_collections_by(&exist_accounts.get(0).unwrap().id.to_string()).await;
    Ok(Json(result))
}

/// 我的专辑(分页查询)
pub async fn get_my_collections(State(config): State<Arc<ServerConfig>>, claims: Claims, Query(args): Query<PageQueryArgs>) -> Result<Json<CollectionPageDTOList>, (StatusCode, String)> {
    let exist_accounts = account_repository::find_by_pubkey(&claims.pubkey).await;
    if exist_accounts.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "未知账户".to_owned()));
    }
    let page = if args.page.is_none() || args.page.unwrap() < 1 {1} else {args.page.unwrap()};
    let page_size = if args.page_size.is_none() || args.page_size.unwrap() < 1 {10} else {args.page_size.unwrap()};
    let account_id = exist_accounts.get(0).unwrap().id;
    let page_info = my_collection_query_service::my_collections(account_id.to_string(), page as u64, page_size as u64, &config.assets_http_addr).await;
    let total = page_info.1;
    Ok(Json(CollectionPageDTOList{
        dtos: page_info.0,
        page_info: PageInfo{
            total: total,
            pages: (total + page_size as u64 - 1) / page_size as u64
        }
    }))
}

/// 创建图文
pub async fn create_article(claims: Claims, Json(payload): Json<ArticleDTO>) -> impl IntoResponse {
    tracing::debug!("{}", serde_json::to_string(&claims).unwrap());
    tracing::debug!("{}", serde_json::to_string(&payload).unwrap());
    let validate_result = validate_request_id(&payload.request_id);
    if validate_result.is_err() {
        return validate_result.err().unwrap();
    }

    let command = CreateArticleCommand {
        title: payload.title,
        description: payload.description,
        is_public: 1,
        pub_key: claims.pubkey,
        collection_id: payload.collection_id,
        content: payload.content,
    };
    let application_result = collection_application_service::create_article(command).await;
    if application_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, application_result.err().unwrap().to_string());
    }
    (StatusCode::OK, application_result.unwrap())
}

/// 获取专辑详细信息,包括专辑包括的所有内容(目前只有图文)
pub async fn get_my_collection_info_by_id(State(config): State<Arc<ServerConfig>>, claims: Claims, Path(collection_id): Path<String>) -> Result<Json<CollectionInfoDTO>, (StatusCode, String)> {
    let exist_accounts = account_repository::find_by_pubkey(&claims.pubkey).await;
    if exist_accounts.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "未知账户".to_owned()));
    }
    let response_result = my_collection_query_service::get_my_collection_by(&collection_id, &exist_accounts.get(0).unwrap().id.to_string(), &config.assets_http_addr, &config.medias_http_addr).await;
    if response_result.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, response_result.err().unwrap().to_string()));
    }
    Ok(Json(response_result.unwrap()))
}

// /// 创建图集
// pub async fn create_image_gallery(Json(payload): Json<ImageGalleryDTO>) -> impl IntoResponse {
//     // todo!()
//     (StatusCode::OK, "")
// }

/// 添加视频
pub async fn add_video(State(state): State<Arc<ServerConfig>>, claims: Claims, Json(payload): Json<AddVideoPayload>) -> impl IntoResponse {
    let command = AddVideoCommand {
        collection_id: payload.collection_id,
        title: payload.title,
        description: payload.description,
        is_public: payload.is_public,
        video_path: payload.video_path,
        hash: payload.file_hash,
        pub_key: claims.pubkey,
    };
    let add_video_result = collection_application_service::add_video(&command, &state.medias_path).await;
    if add_video_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, add_video_result.err().unwrap().to_string())
    }
    (StatusCode::OK, "success".to_owned())
}

// /// 创建音频
// pub async fn create_audio(Json(payload): Json<AudioDTO>) -> impl IntoResponse {
//     // todo!()
//     (StatusCode::OK, "")
// }

// /// 创建文件夹
// pub async fn create_folder(Json(payload): Json<FolderDTO>) -> impl IntoResponse {
//     // todo!()
//     (StatusCode::OK, "")
// }