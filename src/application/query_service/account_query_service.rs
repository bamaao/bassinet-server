use anyhow::Ok;
use sea_orm::EntityTrait;
use std::{collections::HashSet, str::FromStr};

use crate::{domain::{model::entity::prelude::Account, repository::{account_repository, bassinet_coin_repository, collection_repository}}, infrastructure::database_connection::get_db, interface::rest::dto::account::AccountInfo};

/// 获取账户信息
pub async fn get_account_info(pub_key: &String) -> Result<AccountInfo, anyhow::Error> {
    let account = account_repository::get_account_by(pub_key).await?;
    let account_id = account.id.to_string();

    let coin = bassinet_coin_repository::get_coin_by_account_id(&account_id).await?;

    Ok(AccountInfo {
        account_id: account_id,
        nick_name: account.nick_name.unwrap(),
        avatar: account.avatar,
        wallet_address: account.wallet_address,
        package_id: if coin.is_some() {Some(coin.unwrap().package_id)} else {None},
    })
}

/// 获取所有有专辑的账户,测试用
pub async fn get_authors() -> Result<Vec<AccountInfo>, anyhow::Error> {
    let collections = collection_repository::get_all_collections().await?;
    if collections.is_empty() {
        return Ok(Vec::new())
    }
    let mut author_ids = HashSet::new();
    for item in collections.iter() {
        author_ids.insert(item.author.clone());
    }
    let mut results = Vec::new();
    for author_id in author_ids.iter() {
        let author = Account::find_by_id(uuid::Uuid::from_str(author_id).unwrap()).one(get_db().as_ref()).await?;
        if author.is_some() {
            let author = author.unwrap();
            results.push(AccountInfo {
                account_id: author.id.to_string(),
                nick_name: author.nick_name.unwrap(),
                avatar: author.avatar,
                wallet_address: None,
                package_id: None
            });
        }
    }

    Ok(results)
}
