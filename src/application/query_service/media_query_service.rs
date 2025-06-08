use redis::AsyncCommands;
use uuid::Uuid;

use crate::{domain::{model::entity::collection::Model as CollectionModel, repository::{bassinet_nft_repository}}, infrastructure::{redis_connection, sui::nft_query}};

/// 获取ViewingKey
/// 是否可查看: 
/// 1. 视频所属专辑是公开的且视频是公开的
/// 2. 视频所属专辑非公开但已经Minting相应NFT
pub async fn viewing_key(author_id: String, wallet_address: Option<String>, collection: &CollectionModel)-> Option<String> {
    // 专辑公开
    if collection.is_public == 1 || author_id == collection.author{
        let mut redis_connection = redis_connection::get_redis_connection().await;
        let viewing_key = Uuid::new_v4().to_string();
        let key: &str = &("viewing_key_".to_owned() + viewing_key.as_str());
        let _value = redis_connection.set_ex::<&str, u64, u64>(key, 1u64, 2 * 60 * 60).await;
        return Some(viewing_key)
    } else if collection.listing.is_some() && collection.listing.unwrap() == 1 && wallet_address.is_some(){//已经上架
        let nft = bassinet_nft_repository::get_nft_by_collection_id(&collection.id.to_string()).await;
        if nft.is_some() {
            let nft = nft.unwrap();
            let package_id = nft.package_id;
            let result = nft_query::get_any_bassinet_nft_by(&wallet_address.unwrap(), &package_id).await;
            if result.is_ok() {
                let object = result.unwrap();
                if object.is_some() {
                    let mut redis_connection = redis_connection::get_redis_connection().await;
                    let viewing_key = Uuid::new_v4().to_string();
                    let key: &str = &("viewing_key_".to_owned() + viewing_key.as_str());
                    let _value = redis_connection.set_ex::<&str, u64, u64>(key, 1u64, 2 * 60 * 60).await;
                    return Some(viewing_key)
                }
            }
        }
    }
    None
}