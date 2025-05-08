
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::{domain::{model::entity::collection, repository::collection_repository::{self}}, infrastructure::database_connection, interface::rest::dto::collection::{ArticleInfoDTO, CollectionInfoDTO, CollectionPageDTO}};

/// 专辑详情(公开专辑)
pub async fn get_collection_by_id(collection_id: &String) -> Result<CollectionInfoDTO, anyhow::Error> {
    let collection = collection_repository::get_by_id(collection_id).await;
    if collection.is_none() {
        return anyhow::bail!("未知专辑");
    }
    let collection = collection.unwrap();
    if collection.is_public != 1 {
        return anyhow::bail!("未知专辑");
    }
    let articles = collection_repository::get_articles_by(collection_id).await;
    if articles.is_err() {
        return Err(articles.err().unwrap());
    }
    let article_dtos = articles.unwrap().into_iter().map(|item|{
        ArticleInfoDTO{
            id: item.id.to_string(),
            title: item.title.unwrap(),
            collection_id: item.collection_id,
            description: if item.description.is_none() {"".to_owned()} else { item.description.unwrap()},
            content: item.content.unwrap(),
            content_type: item.category,
            created_time: item.created_time.and_utc().timestamp() as u64,
        }
    }).collect();

    Ok(CollectionInfoDTO {
        id: collection_id.clone(),
        title: collection.title,
        description: collection.description,
        is_public: collection.is_public as u8,
        created_time: collection.created_time.and_utc().timestamp() as u64,
        articles: article_dtos,
    })
}

/// 某创作者的专辑分页查询(公开的)
pub async fn get_author_collections(author_id: String, offset: u64, limit: u64) -> Vec<CollectionPageDTO> {
    let db = database_connection::get_db();
    let collection_pages = collection::Entity::find()
    .filter(collection::Column::IsPublic.eq(1))
    .filter(collection::Column::Author.eq(author_id))
    .order_by_desc(collection::Column::CreatedTime)
    .offset(offset).limit(limit).paginate(db.as_ref(), limit);
    
    let collections = collection_pages.fetch().await.unwrap();
    if collections.is_empty() {
        return vec![]
    }
    collections.into_iter().map(|item| {
        CollectionPageDTO{
            id: item.id.to_string(),
            title: item.title,
            description: item.description,
            is_public: item.is_public as u8,
            created_time: item.created_time.and_utc().timestamp() as u64,
        }
    }).collect()
}

/// 某创作者专辑总数(公开的)
pub async fn count_author_collections(author_id: String) -> u64 {
    let db = database_connection::get_db();
    let count = collection::Entity::find().filter(collection::Column::IsPublic.eq(1)).filter(collection::Column::Author.eq(author_id))
    .count(db.as_ref());
    return count.await.unwrap();
}

/// 条件搜索专辑,分页查询(公开的)
pub async fn search_collections(keyword: Option<String>, offset: u64, limit: u64) -> Vec<CollectionPageDTO> {
    let db = database_connection::get_db();

    let collection_pages = if keyword.is_some() {
        let mut like_condition = String::new();
        like_condition.push_str("%");
        like_condition.push_str(keyword.unwrap().as_str());
        like_condition.push_str("%");
        collection::Entity::find()
        .filter(collection::Column::IsPublic.eq(1))
        .filter(collection::Column::Title.like(like_condition))
        .order_by_desc(collection::Column::CreatedTime)
        .offset(offset).limit(limit).paginate(db.as_ref(), limit)
    }else {
        collection::Entity::find()
        .filter(collection::Column::IsPublic.eq(1))
        .order_by_desc(collection::Column::CreatedTime)
        .offset(offset).limit(limit).paginate(db.as_ref(), limit)
    };
    
    let collections = collection_pages.fetch().await.unwrap();
    if collections.is_empty() {
        return vec![]
    }
    collections.into_iter().map(|item| {
        CollectionPageDTO{
            id: item.id.to_string(),
            title: item.title,
            description: item.description,
            is_public: item.is_public as u8,
            created_time: item.created_time.and_utc().timestamp() as u64,
        }
    }).collect()
}

/// 条件搜索专辑总数(公开的)
pub async fn count_search_collections(keyword: Option<String>) -> u64 {
    let db = database_connection::get_db();

    let count = if keyword.is_some() {
        let mut like_condition = String::new();
        like_condition.push_str("%");
        like_condition.push_str(keyword.unwrap().as_str());
        like_condition.push_str("%");
        collection::Entity::find()
        .filter(collection::Column::IsPublic.eq(1))
        .filter(collection::Column::Title.like(like_condition))
        .count(db.as_ref())
    }else {
        collection::Entity::find()
        .filter(collection::Column::IsPublic.eq(1))
        .count(db.as_ref())
    };
    return count.await.unwrap();
}

/// 获取公开的图文
/// TODO 是否公开应该从专辑是否公开来判断
pub async fn get_article_by_id(article_id: String) -> Result<ArticleInfoDTO, anyhow::Error> {
    let article = collection_repository::get_article_by_id(&article_id).await;
    if article.is_none(){
        return anyhow::bail!("未知图文".to_owned());
    }
    let article = article.unwrap();
    if article.is_public != 1 {
        return anyhow::bail!("未知图文".to_owned());
    }
    Ok(ArticleInfoDTO{
        id: article.id.to_string(),
        title: article.title.unwrap(),
        collection_id: article.collection_id,
        description: if article.description.is_none() {"".to_owned()} else {article.description.unwrap()},
        content: if article.content.is_none() {"".to_owned()} else {article.content.unwrap()},
        content_type: "Markdown".to_owned(),
        created_time: article.created_time.and_utc().timestamp() as u64,
    })
}