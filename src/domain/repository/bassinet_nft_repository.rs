use sea_orm::{ColumnTrait, DbBackend, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::{domain::model::entity::{bassinet_nft, prelude::BassinetNft}, infrastructure::database_connection::get_db};

/// 根据package_id获取NFT信息
pub async fn get_nft_by_package_id(package_id: &String) -> Option<bassinet_nft::Model> {
    let nfts = BassinetNft::find().filter(bassinet_nft::Column::PackageId.eq(package_id))
    .all(get_db().as_ref()).await.unwrap();

    if nfts.is_empty() {
        return Option::None
    }
    let nft = nfts.get(0).unwrap().clone();
    Some(nft)
}

/// 根据package_id获取NFT信息
pub async fn get_nft_by_collection_id(collection_id: &String) -> Option<bassinet_nft::Model> {
    let nfts = BassinetNft::find().filter(bassinet_nft::Column::CollectionId.eq(collection_id))
    .all(get_db().as_ref()).await.unwrap();

    if nfts.is_empty() {
        return Option::None
    }
    let nft = nfts.get(0).unwrap().clone();
    Some(nft)
}

// /// 根据collection_id集获取nft集
// pub async fn get_nft_by_collection_ids(collection_ids: &Vec<String>) -> Result<Vec<bassinet_nft::Model>, anyhow::Error> {
//     let collections = bassinet_nft::Model::find_by_statement(
//         Statement::from_sql_and_values(
//             DbBackend::Postgres, 
//             r#"SELECT * FROM "bassinet_nft WHERE "collection_id" in ($1)"#, 
//             [collection_ids.clone().into()],
//         )
//     ).all(get_db().as_ref()).await?;
//     Ok(collections)
// }
