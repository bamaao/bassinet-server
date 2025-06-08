use anyhow::Ok;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;
use crate::{domain::model::entity::{collection, collection_item, prelude::{Collection, CollectionItem}}, infrastructure::database_connection::{self, get_db}};

/// 根据collection_id获取专辑 
pub async fn get_by_id(collection_id: &String) -> Option<collection::Model> {
    Collection::find_by_id(Uuid::parse_str(&collection_id).unwrap()).one(database_connection::get_db().as_ref()).await.unwrap()
}

/// 根据collection_id和author_id获取我的专辑
pub async fn get_my_collection_by_id(collection_id: &String, author_id: &String) -> Option<collection::Model> {
    let opt = Collection::find_by_id(Uuid::parse_str(&collection_id).unwrap()).one(database_connection::get_db().as_ref()).await.unwrap();
    if opt.is_some() {
        let collection = opt.unwrap();
        if collection.author.eq(author_id) && collection.status == 1 {
            return Option::Some(collection);
        }
    }
    Option::None
}

/// 根据Author获取专辑列表
pub async fn get_by_author(author_id: &String) -> Vec<crate::domain::model::entity::collection::Model> {
    let db = database_connection::get_db();
    Collection::find().filter(collection::Column::Author.eq(author_id))
    .filter(collection::Column::IsPublic.eq(1))
    .order_by_desc(collection::Column::CreatedTime)
    .all(db.as_ref())
    .await.expect("Database error")
}

/// 创建专辑
pub async fn create_collection(collection: collection::ActiveModel) -> Result<(), anyhow::Error> {
    collection.insert(database_connection::get_db().as_ref()).await?;
    Ok(())
}

/// 根据title搜索我的专辑
pub async fn search_collection_by(title: &String, author_id: &String) -> Result<Vec<crate::domain::model::entity::collection::Model>, anyhow::Error> {
    let results = Collection::find().filter(collection::Column::Author.eq(author_id))
    .filter(Condition::all().add(collection::Column::Title.eq(title)).add(collection::Column::Status.eq(1)))
    .all(database_connection::get_db().as_ref())
    .await;
    if results.is_err() {
        return Err(results.err().unwrap().into());
    }
    Ok(results.unwrap())
}

/// 创建collection item
pub async fn create_collection_item(collection_item: collection_item::ActiveModel) -> Result<(), anyhow::Error> {
    collection_item.insert(database_connection::get_db().as_ref()).await?;
    Ok(())
}

// /// 专辑所有图文
// pub async fn get_articles_by(collection_id: &String) -> Result<Vec<crate::domain::model::entity::collection_item::Model>, anyhow::Error> {
//     let results = CollectionItem::find().filter(collection_item::Column::CollectionId.eq(collection_id))
//     .filter(collection_item::Column::Status.eq(1))
//     .order_by_desc(collection_item::Column::CreatedTime)
//     .all(database_connection::get_db().as_ref())
//     .await;
//     if results.is_err() {
//         return Err(results.err().unwrap().into());
//     }
//     Ok(results.unwrap())
// }

/// 专辑所有内容
pub async fn get_items_by(collection_id: &String) -> Result<Vec<crate::domain::model::entity::collection_item::Model>, anyhow::Error> {
    let results = CollectionItem::find().filter(collection_item::Column::CollectionId.eq(collection_id))
    .filter(collection_item::Column::Status.eq(1))
    .order_by_desc(collection_item::Column::CreatedTime)
    .all(database_connection::get_db().as_ref())
    .await;
    if results.is_err() {
        return Err(results.err().unwrap().into());
    }
    Ok(results.unwrap())
}

/// 图文
pub async fn get_article_by_id(article_id: &String) -> Option<crate::domain::model::entity::collection_item::Model> {
    let opt = CollectionItem::find_by_id(Uuid::parse_str(&article_id).unwrap()).one(database_connection::get_db().as_ref()).await.unwrap();
    if opt.is_some() {
        let article = opt.unwrap();
        if article.category == "article" {
            return Some(article);
        }
    }
    Option::None
}

/// 专辑项
pub async fn get_item_by(item_id: &String) -> Option<crate::domain::model::entity::collection_item::Model> {
    CollectionItem::find_by_id(Uuid::parse_str(&item_id).unwrap()).one(database_connection::get_db().as_ref()).await.unwrap()
}

pub async fn get_all_collections() -> Result<Vec<collection::Model>, anyhow::Error> {
    let collections = Collection::find().filter(Condition::any().add(collection::Column::IsPublic.eq(1)).add(collection::Column::Listing.eq(1)))
    .order_by_desc(collection::Column::CreatedTime)
    .all(get_db().as_ref())
    .await;
    if collections.is_err() {
        return Err(collections.err().unwrap().into())
    }
    Ok(collections.unwrap())
}