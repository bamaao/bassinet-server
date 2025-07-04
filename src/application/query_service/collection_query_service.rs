
use anyhow::Ok;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::{domain::{model::entity::collection, repository::{bassinet_nft_repository, collection_repository::{self}}}, infrastructure::database_connection, interface::rest::dto::{account::AccountInfo, collection::{ArticleInfoDTO, CollectionInfoDTO, CollectionItemInfoDTO, CollectionPageDTO, CollectionSimpleInfoDTO, NftInfo}}};

use super::media_query_service;

/// 专辑详情(公开专辑)
pub async fn get_collection_by_id(collection_id: &String, assets_web_addr: &String, medias_web_addr: &String) -> Result<CollectionInfoDTO, anyhow::Error> {
    let collection = collection_repository::get_by_id(collection_id).await;
    if collection.is_none() {
        anyhow::bail!("未知专辑");
    }
    let collection = collection.unwrap();
    if collection.is_public != 1 && collection.listing.unwrap() != 1 {
        anyhow::bail!("未知专辑");
    }
    let items = collection_repository::get_items_by(collection_id).await;
    if items.is_err() {
        return Err(items.err().unwrap());
    }
    let dtos = items.unwrap().into_iter().map(|item|{
        CollectionItemInfoDTO{
            id: item.id.to_string(),
            title: item.title.unwrap(),
            collection_id: item.collection_id,
            description: if item.description.is_none() {"".to_owned()} else { item.description.unwrap()},
            content: if item.content.is_none() {"".to_owned()} else {item.content.unwrap()},
            category: item.category,
            url_path: if item.path.is_none() {"".to_owned()} else {format!("{}/{}", medias_web_addr, item.path.unwrap())},
            content_type: "".to_owned(),
            created_time: item.created_time.and_utc().timestamp() as u64,
        }
    }).collect();

    let nft = bassinet_nft_repository::get_nft_by_collection_id(&collection_id).await;
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
        icon_url: Some(assets_web_addr.clone() + &collection.icon_url.unwrap()),
        nft: nft_dto,
        items: dtos
        // articles: article_dtos,
    })
}

/// 某创作者的专辑分页查询(公开的)
pub async fn get_author_collections(author_id: String, page: u64, limit: u64, assets_path: &String) -> (Vec<CollectionPageDTO>, u64) {
    let db = database_connection::get_db();
    let collection_pages = collection::Entity::find()
    .filter(
        Condition::any()
                .add(collection::Column::IsPublic.eq(1))
                .add(collection::Column::Listing.eq(1))
    )
    .filter(collection::Column::Author.eq(author_id))
    .order_by_desc(collection::Column::CreatedTime)
    .paginate(db.as_ref(), limit);
    
    let collections = collection_pages.fetch_page(page - 1).await.unwrap();
    if collections.is_empty() {
        return (vec![], 0)
    }
    let total = collection_pages.num_items().await.unwrap();
    let mut values = Vec::new();
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
    (values, total)
}

/// 条件搜索专辑,分页查询(公开的)
pub async fn search_collections(keyword: Option<String>, author: Option<String>, page: u64, limit: u64, assets_path: &String) -> (Vec<CollectionPageDTO>, u64) {
    let db = database_connection::get_db();

    let collection_pages = if keyword.is_some() && author.is_some() {
        let mut like_condition = String::new();
        like_condition.push_str("%");
        like_condition.push_str(keyword.unwrap().as_str());
        like_condition.push_str("%");
        collection::Entity::find()
        .filter(
            Condition::all().add(
                Condition::any()
                .add(collection::Column::IsPublic.eq(1))
                .add(collection::Column::Listing.eq(1))
            ).add(collection::Column::Title.like(like_condition))
            .add(collection::Column::Author.eq(author.unwrap()))   
        )
        .order_by_desc(collection::Column::CreatedTime)
        .paginate(db.as_ref(), limit)
    }else if keyword.is_some() {
        let mut like_condition = String::new();
        like_condition.push_str("%");
        like_condition.push_str(keyword.unwrap().as_str());
        like_condition.push_str("%");
        collection::Entity::find()
        .filter(
            Condition::all().add(
                Condition::any()
            .add(collection::Column::IsPublic.eq(1))
            .add(collection::Column::Listing.eq(1))
            ).add(collection::Column::Title.like(like_condition))
        )
        .order_by_desc(collection::Column::CreatedTime)
        .paginate(db.as_ref(), limit)
    }else if author.is_some() {
        collection::Entity::find()
        .filter(
            Condition::all()
            .add(Condition::any().add(collection::Column::IsPublic.eq(1)).add(collection::Column::Listing.eq(1)))
            .add(collection::Column::Author.eq(author.unwrap()))
        )
        .order_by_desc(collection::Column::CreatedTime)
        .paginate(db.as_ref(), limit)
    }else {
        collection::Entity::find()
        .filter(Condition::any().add(collection::Column::IsPublic.eq(1)).add(collection::Column::Listing.eq(1)))
        .order_by_desc(collection::Column::CreatedTime)
        .paginate(db.as_ref(), limit)
    };
    
    let collections = collection_pages.fetch_page(page - 1).await.unwrap();
    if collections.is_empty() {
        return (vec![], 0)
    }
    let total = collection_pages.num_items().await.unwrap();
    let mut values = Vec::new();
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
    (values, total)
}

/// 获取公开的图文
/// TODO 是否公开应该从专辑是否公开来判断
pub async fn get_article_by_id(article_id: String) -> Result<ArticleInfoDTO, anyhow::Error> {
    let article = collection_repository::get_article_by_id(&article_id).await;
    if article.is_none(){
        anyhow::bail!("未知图文".to_owned());
    }
    let article = article.unwrap();
    if article.is_public != 1 {
        anyhow::bail!("未知图文".to_owned());
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

pub async fn get_video_by_id(video_id: String, medias_web_addr: &String, account: AccountInfo) -> Result<CollectionItemInfoDTO, anyhow::Error> {
    let item = collection_repository::get_item_by(&video_id).await;
    if item.is_none() {
        anyhow::bail!("未知视频");
    }
    let video = item.unwrap();
    if video.category != "video" {
        anyhow::bail!("未知视频");
    }
    let collection = collection_repository::get_by_id(&video.collection_id).await;
    if collection.is_none() {
        anyhow::bail!("未知视频");
    }

    let viewing_key = media_query_service::viewing_key(account.account_id, account.wallet_address, &collection.unwrap()).await;
    if viewing_key.is_none() {
        anyhow::bail!("无法访问该视频");
    }
    Ok(CollectionItemInfoDTO { 
        id: video_id, 
        title: video.title.unwrap(), 
        collection_id: video.collection_id, 
        description: video.description.unwrap(), 
        content: "".to_owned(), 
        category: video.category, 
        url_path: format!("{}/{}?viewingKey={}", medias_web_addr, video.path.unwrap(), viewing_key.unwrap()), 
        content_type: "".to_owned(), 
        created_time: video.created_time.and_utc().timestamp() as u64 })
}

pub async fn get_collection_simple_info_by_id(collection_id: &String, assets_path: &String) -> Result<CollectionSimpleInfoDTO, anyhow::Error> {
    let collection = collection_repository::get_by_id(collection_id).await;
    if collection.is_none() {
        anyhow::bail!("未知专辑");
    }
    let collection = collection.unwrap();
    let collection_url = assets_path.to_owned() + "/" + &collection_id;
    Ok(CollectionSimpleInfoDTO {
        id: collection_id.clone(),
        title: collection.title,
        description: collection.description,
        collection_url: collection_url,
    })
}