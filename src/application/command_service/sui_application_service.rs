use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{domain::{model::entity::{account, bassinet_coin, bassinet_nft, collection}, repository::{account_repository, bassinet_coin_repository, collection_repository}}, infrastructure::{database_connection::{self, get_db}, messaging::{coin_published_consumer::CoinPublishedMessage, nft_published_consumer::NftPublishedMessage}}};

pub async fn add_bassinet_coin(coin_info: &CoinPublishedMessage) -> Result<(), anyhow::Error> {
    let accounts = account_repository::find_by_pubkey(&coin_info.account).await;
    let account = accounts.get(0);
    let account_id = account.unwrap().id;
    let id = uuid::Uuid::new_v4();
    let bassinet_coin = bassinet_coin::ActiveModel {
        id: Set(id),
        package_id: Set(coin_info.package_id.clone()),
        symbol: Set(coin_info.symbol.clone()),
        name: Set(coin_info.name.clone()),
        description: Set(Some(coin_info.description.clone())),
        icon_url: Set(Some(coin_info.icon_url.clone())),
        treasury_lock_id: Set(coin_info.treasury_lock_id.clone()),
        admin_cap_id: Set(coin_info.admin_cap_id.clone()),
        account_id: Set(Some(account_id.to_string())),
    };
    bassinet_coin.insert(database_connection::get_db().as_ref()).await?;
    Ok(())
}

/// 确认绑定钱包
pub async fn confirm_account_bound(pub_key: &String, wallet_address: String) {
    let accounts = account_repository::find_by_pubkey(pub_key).await;
    if accounts.len() > 0 {
        let account = accounts.get(0).unwrap().clone();
        let mut sui_account:account::ActiveModel = account.into();
        sui_account.wallet_address = Set(Some(wallet_address));
        sui_account.update(get_db().as_ref()).await.expect("Database error");
    }
}

pub async fn add_bassinet_nft(nft_info: &NftPublishedMessage) -> Result<(), anyhow::Error> {
    let coin = bassinet_coin_repository::get_coin_by_package_id(&nft_info.coin_package_id).await;
    let id = uuid::Uuid::new_v4();
    let bassinet_nft = bassinet_nft::ActiveModel {
        id: Set(id),
        package_id: Set(nft_info.package_id.clone()),
        collection_id: Set(nft_info.collection_id.clone()),
        description: Set(Some(nft_info.description.clone())),
        collection_url: Set(Some(nft_info.collection_url.clone())),
        limit: Set(Some(nft_info.limit.try_into().unwrap())),
        minting_price: Set(nft_info.minting_price.try_into().unwrap()),
        rewards_quantity: Set(Some(nft_info.rewards_quantity.try_into().unwrap())),
        mint_id: Set(Some(nft_info.mint_id.clone())),
        policy_id: Set(Some(nft_info.policy_id.clone())),
        policy_cap_id: Set(Some(nft_info.policy_cap_id.clone().clone())),
        coin_id: Set(Some(coin.unwrap().id.to_string())),
        coin_package_id: Set(Some(nft_info.coin_package_id.clone())),
        coin_treasury_lock_id: Set(Some(nft_info.treasury_lock_id.clone())),
        coin_admin_cap_id: Set(Some(nft_info.admin_cap_id.clone())),
    };
    bassinet_nft.insert(database_connection::get_db().as_ref()).await?;

    let collection = collection_repository::get_by_id(&nft_info.collection_id).await;
    if collection.is_some() {
        let mut updated : collection::ActiveModel = collection.unwrap().into();
        updated.listing = Set(Some(1));
        let _ = updated.update(get_db().as_ref()).await;
    }
    Ok(())
}