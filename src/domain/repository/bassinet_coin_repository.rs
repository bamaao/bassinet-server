use anyhow::Ok;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{domain::model::entity::{bassinet_coin, prelude::BassinetCoin}, infrastructure::database_connection::get_db};

/// 根据package_id获取BassinetCoin
pub async fn get_coin_by_package_id(package_id: &String) -> Option<bassinet_coin::Model> {
    let coins = BassinetCoin::find().filter(bassinet_coin::Column::PackageId.eq(package_id))
    .all(get_db().as_ref()).await.unwrap();

    if coins.is_empty() {
        return Option::None
    }
    let coin = coins.get(0).unwrap().clone();
    Some(coin)
}

/// 根据account_id获取Bassinet Coin
pub async  fn get_coin_by_account_id(account_id: &String) -> Result<Option<bassinet_coin::Model>, anyhow::Error> {
    let coins = BassinetCoin::find().filter(bassinet_coin::Column::AccountId.eq(account_id))
    .all(get_db().as_ref()).await?;

    if coins.is_empty() {
        return Ok(Option::None)
    }

    Ok(Some(coins.get(0).unwrap().clone()))
}