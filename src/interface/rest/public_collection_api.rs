use axum::{extract::{Path, Query}, http::StatusCode, Json};

use crate::application::query_service::collection_query_service;

use super::dto::{collection::{ArticleInfoDTO, CollectionInfoDTO, CollectionPageDTOList, PageInfo}, PageQueryArgs};

/// 获取专辑详细信息,包括专辑包括的所有内容(目前只有图文)
pub async fn get_collection_info_by_id(Path(collection_id): Path<String>) -> Result<Json<CollectionInfoDTO>, (StatusCode, String)> {
    let response_result = collection_query_service::get_collection_by_id(&collection_id).await;
    if response_result.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, response_result.err().unwrap().to_string()));
    }
    Ok(Json(response_result.unwrap()))
}

/// 某创作者的专辑(分页查询)
pub async fn get_author_collections(Path(author_id): Path<String>, Query(args): Query<PageQueryArgs>) -> Result<Json<CollectionPageDTOList>, (StatusCode, String)> {
    let page = if args.page.is_none() || args.page.unwrap() < 1 {1} else {args.page.unwrap()};
    let page_size = if args.page_size.is_none() || args.page_size.unwrap() < 1 {10} else {args.page_size.unwrap()};
    let offset = ((page - 1) * page_size) as u64;
    let offset = if offset < 1 {0} else {offset};
    let dtos = collection_query_service::get_author_collections(author_id.clone(), offset, page_size as u64).await;
    let total = collection_query_service::count_author_collections(author_id).await;
    Ok(Json(CollectionPageDTOList{
        dtos: dtos,
        page_info: PageInfo{
            total: total,
            pages: (total + 9) / 10
        }
    }))
}

/// 搜索专辑(分页查询)
pub async fn search_collections(Query(args): Query<PageQueryArgs>) -> Result<Json<CollectionPageDTOList>, (StatusCode, String)> {
    let page = if args.page.is_none() || args.page.unwrap() < 1 {1} else {args.page.unwrap()};
    let page_size = if args.page_size.is_none() || args.page_size.unwrap() < 1 {10} else {args.page_size.unwrap()};
    let offset = ((page - 1) * page_size) as u64;
    let offset = if offset < 1 {0} else {offset};
    let dtos = collection_query_service::search_collections(args.keyword.clone(), offset, page_size as u64).await;
    let total = collection_query_service::count_search_collections(args.keyword).await;
    Ok(Json(CollectionPageDTOList { dtos: dtos, page_info: PageInfo { total: total, pages: (total + 9) / 10 } }))
}

/// 获取文章详情
pub async fn get_article_by_id(Path(article_id): Path<String>) -> Result<Json<ArticleInfoDTO>, (StatusCode, String)> {
    let article = collection_query_service::get_article_by_id(article_id).await;
    if article.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, article.err().unwrap().to_string()));
    }
    Ok(Json(article.unwrap()))
}