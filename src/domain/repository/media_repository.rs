// use sea_orm::EntityTrait;
// use uuid::Uuid;

// use crate::{domain::model::entity::{collection_item, prelude::{CollectionItem}}, infrastructure::database_connection};

// /// 根据id获取media
// pub async fn get_by_id(media_id: &String, collection_id: &String) -> Option<collection_item::Model> {
//     let result = CollectionItem::find_by_id(Uuid::parse_str(&media_id).unwrap()).one(database_connection::get_db().as_ref()).await.unwrap();
//     if result.is_none() {
//         return None
//     }
//     let media = result.unwrap();
//     if media.collection_id == *collection_id && media.category == "video" {
//         return Some(media)
//     }
//     return None
// }