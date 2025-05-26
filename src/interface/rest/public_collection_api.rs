use std::{path::Path as FilePath, sync::Arc};

use axum::{body::Body, extract::{Path, Query, State}, http::{header::{CONTENT_TYPE}, StatusCode}, response::{IntoResponse, Response}, Json};
use tokio::fs::{self, File};
use tokio_util::io::ReaderStream;

use crate::{application::query_service::collection_query_service, domain::repository::collection_repository, infrastructure::image_util::{image_type, make_thumbnail}, ServerConfig};

use super::dto::{collection::{ArticleInfoDTO, CollectionInfoDTO, CollectionPageDTOList, CollectionSimpleInfoDTO, PageInfo}, PageQueryArgs};

/// 获取专辑详细信息,包括专辑包括的所有内容(目前只有图文)
pub async fn get_collection_info_by_id(State(config): State<Arc<ServerConfig>>, Path(collection_id): Path<String>) -> Result<Json<CollectionInfoDTO>, (StatusCode, String)> {
    let response_result = collection_query_service::get_collection_by_id(&collection_id, &config.assets_http_addr).await;
    if response_result.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, response_result.err().unwrap().to_string()));
    }
    Ok(Json(response_result.unwrap()))
}

/// 某创作者的专辑(分页查询)
pub async fn get_author_collections(State(config): State<Arc<ServerConfig>>, Path(author_id): Path<String>, Query(args): Query<PageQueryArgs>) -> Result<Json<CollectionPageDTOList>, (StatusCode, String)> {
    let page = if args.page.is_none() || args.page.unwrap() < 1 {1} else {args.page.unwrap()};
    let page_size = if args.page_size.is_none() || args.page_size.unwrap() < 1 {10} else {args.page_size.unwrap()};
    let offset = ((page - 1) * page_size) as u64;
    let offset = if offset < 1 {0} else {offset};
    let dtos = collection_query_service::get_author_collections(author_id.clone(), offset, page_size as u64, &config.assets_http_addr).await;
    let total = collection_query_service::count_author_collections(author_id).await;
    Ok(Json(CollectionPageDTOList{
        dtos: dtos,
        page_info: PageInfo{
            total: total,
            pages: (total + page_size as u64 - 1) / page_size as u64
        }
    }))
}

/// 搜索专辑(分页查询)
pub async fn search_collections(State(config): State<Arc<ServerConfig>>, Query(args): Query<PageQueryArgs>) -> Result<Json<CollectionPageDTOList>, (StatusCode, String)> {
    let page = if args.page.is_none() || args.page.unwrap() < 1 {1} else {args.page.unwrap()};
    let page_size = if args.page_size.is_none() || args.page_size.unwrap() < 1 {10} else {args.page_size.unwrap()};
    let offset = ((page - 1) * page_size) as u64;
    let offset = if offset < 1 {0} else {offset};
    let dtos = collection_query_service::search_collections(args.keyword.clone(), args.author.clone(), offset, page_size as u64, &config.assets_http_addr).await;
    let total = collection_query_service::count_search_collections(args.keyword, args.author).await;
    Ok(Json(CollectionPageDTOList { dtos: dtos, page_info: PageInfo { total: total, pages: (total + page_size as u64 - 1) / page_size as u64 } }))
}

/// 获取文章详情
pub async fn get_article_by_id(Path(article_id): Path<String>) -> Result<Json<ArticleInfoDTO>, (StatusCode, String)> {
    let article = collection_query_service::get_article_by_id(article_id).await;
    if article.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, article.err().unwrap().to_string()));
    }
    Ok(Json(article.unwrap()))
}

/// 获取集合简要信息
pub async fn get_collection_simple_by_id(State(config): State<Arc<ServerConfig>>, Path(collection_id): Path<String>) -> Result<Json<CollectionSimpleInfoDTO>, (StatusCode, String)> {
    let response_result: Result<CollectionSimpleInfoDTO, anyhow::Error> = collection_query_service::get_collection_simple_info_by_id(&collection_id, &config.assets_http_addr).await;
    if response_result.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, response_result.err().unwrap().to_string()));
    }
    Ok(Json(response_result.unwrap()))
}

/// 获取专辑图片
pub async fn get_image(State(config): State<Arc<ServerConfig>>, Path(collection_id): Path<String>) -> impl IntoResponse {
    let collection = collection_repository::get_by_id(&collection_id).await;
    if collection.is_none() {
        return StatusCode::NOT_FOUND.into_response()
    }
    let collection = collection.unwrap();

    let file_path = config.assets_path.clone() + &collection.icon_url.unwrap();
    let path = FilePath::new(&file_path);
    let extension = path.extension();
    if extension.is_none() {
        return StatusCode::NOT_FOUND.into_response()
    }
    let extension = extension.unwrap().to_str().unwrap();
    let image_type = image_type(extension);
    if image_type.is_none() {
        return StatusCode::NOT_FOUND.into_response()
    }
    let file =  File::open(&file_path).await;
    if file.is_err() {
        return StatusCode::NOT_FOUND.into_response()
    }
    let file = file.unwrap();
    let stream = ReaderStream::new(file);
    Response::builder()
    .header(CONTENT_TYPE, image_type.unwrap())
    .body(Body::from_stream(stream))
    .unwrap()
}

/// 获取专辑缩略图
pub async fn get_thumbnail(State(config): State<Arc<ServerConfig>>, Path(collection_id): Path<String>) -> impl IntoResponse {
    let collection = collection_repository::get_by_id(&collection_id).await;
    if collection.is_none() {
        return StatusCode::NOT_FOUND.into_response()
    }

    let file_path = config.assets_path.clone() + &collection.unwrap().icon_url.unwrap();
    let path = FilePath::new(&file_path);
    let file_stem = path.file_stem().unwrap();
    let extension = path.extension().unwrap();
    let image_type = image_type(extension.to_str().unwrap());
    if image_type.is_none() {
        return StatusCode::NOT_FOUND.into_response()
    }
    let thumbnail_file_path = config.assets_path.clone() + "/" + &collection_id + "/" + file_stem.to_str().unwrap() + "_thumb" + "." + extension.to_str().unwrap();
    let file =  File::open(thumbnail_file_path).await;
    // 缩略图不存在，生成缩略图
    if file.is_err() {
        let thumb = make_thumbnail(path).await;
        if thumb.is_none() {
           return StatusCode::NOT_FOUND.into_response()
        }
        let stream = ReaderStream::new(File::open(thumb.unwrap()).await.unwrap());
        return 
        Response::builder()
        .header(CONTENT_TYPE, image_type.unwrap())
        .body(Body::from_stream(stream))
        .unwrap()
    }
    let stream = ReaderStream::new(file.unwrap());
    Response::builder()
    .header(CONTENT_TYPE, image_type.unwrap())
    .body(Body::from_stream(stream))
    .unwrap()
    
}