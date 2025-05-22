use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::{domain::{model::entity::collection, repository::{bassinet_nft_repository, collection_repository}}, infrastructure::database_connection, interface::rest::dto::collection::{ArticleInfoDTO, CollectionInfoDTO, CollectionListDTO, CollectionPageDTO, CollectionSimpleDTO, NftInfo}};

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
pub async fn my_collections(author_id: String, offset: u64, limit: u64, assets_path: &String) -> Vec<CollectionPageDTO> {
    let mut values = Vec::new();

    let db = database_connection::get_db();
    let collection_pages = collection::Entity::find()
    .filter(collection::Column::Author.eq(author_id)).order_by_desc(collection::Column::CreatedTime)
    .offset(offset).limit(limit).paginate(db.as_ref(), limit);
    
    let collections = collection_pages.fetch().await.unwrap();
    if collections.is_empty() {
        return vec![]
    }
    for item in collections.into_iter() {
        let nft = bassinet_nft_repository::get_nft_by_collection_id(&item.id.to_string()).await;
        let mut nft_dto = Option::None;
        if nft.is_some() {
            let nft = nft.unwrap();
            nft_dto = Some(NftInfo {
                id: nft.id.to_string(),
                package_id: nft.package_id,
                collection_url: nft.collection_url.unwrap(),
                limit: nft.limit.unwrap() as u64,
                minting_price: nft.minting_price as u64,
                rewards_quantity: nft.rewards_quantity.unwrap() as u64,
                mint_id: nft.mint_id.unwrap(),
                policy_id: nft.policy_id.unwrap(),
                policy_cap_id: nft.policy_cap_id.unwrap(),
                coin_id: nft.coin_id.unwrap(),
                coin_package_id: nft.coin_package_id.unwrap(),
                coin_treasury_lock_id: nft.coin_treasury_lock_id.unwrap(),
                coin_admin_cap_id: nft.coin_admin_cap_id.unwrap()
            });
        }
        values.push(
            CollectionPageDTO{
                id: item.id.to_string(),
                title: item.title,
                description: item.description,
                is_public: item.is_public as u8,
                listing: item.listing.unwrap() as u8,
                created_time: item.created_time.and_utc().timestamp() as u64,
                icon_url: Some(assets_path.clone() + &item.icon_url.unwrap()),
                nft: nft_dto
        });
    }
    values
}

/// 我的专辑总数
pub async fn count_my_collections(author_id: String) -> u64 {
    let db = database_connection::get_db();
    let count = collection::Entity::find().filter(collection::Column::Author.eq(author_id))
    .count(db.as_ref());
    return count.await.unwrap();
}

/// 专辑详情(我的专辑)
pub async fn get_my_collection_by(collection_id: &String, author_id: &String, assets_path: &String) -> Result<CollectionInfoDTO, anyhow::Error> {
    let collection = collection_repository::get_my_collection_by_id(collection_id, author_id).await;
    if collection.is_none() {
        anyhow::bail!("未知专辑");
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

    let nft = bassinet_nft_repository::get_nft_by_collection_id(collection_id).await;
    let mut nft_dto = Option::None;
    if nft.is_some() {
        let nft = nft.unwrap();
        nft_dto = Some(NftInfo {
            id: nft.id.to_string(),
            package_id: nft.package_id,
            collection_url: nft.collection_url.unwrap(),
            limit: nft.limit.unwrap() as u64,
            minting_price: nft.minting_price as u64,
            rewards_quantity: nft.rewards_quantity.unwrap() as u64,
            mint_id: nft.mint_id.unwrap(),
            policy_id: nft.policy_id.unwrap(),
            policy_cap_id: nft.policy_cap_id.unwrap(),
            coin_id: nft.coin_id.unwrap(),
            coin_package_id: nft.coin_package_id.unwrap(),
            coin_treasury_lock_id: nft.coin_treasury_lock_id.unwrap(),
            coin_admin_cap_id: nft.coin_admin_cap_id.unwrap()
        });
    }

    Ok(CollectionInfoDTO {
        id: collection_id.clone(),
        title: collection.title,
        description: collection.description,
        is_public: collection.is_public as u8,
        listing: collection.listing.unwrap() as u8,
        created_time: collection.created_time.and_utc().timestamp() as u64,
        icon_url: Some(assets_path.clone() + &collection.icon_url.unwrap()),
        nft: nft_dto,
        articles: article_dtos,
    })
}