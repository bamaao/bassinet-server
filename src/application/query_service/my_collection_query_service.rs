use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::{domain::{model::entity::collection, repository::collection_repository}, infrastructure::database_connection, interface::rest::dto::collection::{ArticleInfoDTO, CollectionInfoDTO, CollectionListDTO, CollectionPageDTO, CollectionSimpleDTO}};

/// 根据作者获取专辑列表(简要信息)
pub async fn get_collections_by(author_id: &String) -> CollectionListDTO{
    let list = collection_repository::get_by_author(author_id).await;
    if list.is_empty() {
        return CollectionListDTO {
            collections: vec![],
        };
    }
    let vec = list.iter().map(|element|{ CollectionSimpleDTO{
        id: element.id.to_string(),
        title: element.title.clone(),
    }}).collect();
    CollectionListDTO{
        collections: vec,
    }
}

/// 我的专辑分页查询
pub async fn my_collections(author_id: String, offset: u64, limit: u64) -> Vec<CollectionPageDTO> {
    let db = database_connection::get_db();
    let collection_pages = collection::Entity::find()
    .filter(collection::Column::Author.eq(author_id)).order_by_desc(collection::Column::CreatedTime)
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

/// 我的专辑总数
pub async fn count_my_collections(author_id: String) -> u64 {
    let db = database_connection::get_db();
    let count = collection::Entity::find().filter(collection::Column::Author.eq(author_id))
    .count(db.as_ref());
    return count.await.unwrap();
}

/// 专辑详情(我的专辑)
pub async fn get_my_collection_by(collection_id: &String, author_id: &String) -> Result<CollectionInfoDTO, anyhow::Error> {
    let collection = collection_repository::get_my_collection_by_id(collection_id, author_id).await;
    if collection.is_none() {
        return anyhow::bail!("未知专辑");
    }
    let collection = collection.unwrap();
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