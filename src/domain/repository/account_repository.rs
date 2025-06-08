// /// 根据id获取专辑 
// pub fn get_by_id(account_id: String) -> Option<crate::domain::model::entity::> {
//     Option::None
// }

use anyhow::anyhow;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{domain::model::entity::{account, prelude::Account}, infrastructure::database_connection::get_db};

/// 根据pub_key获取账户信息
pub async fn find_by_pubkey(pub_key: &String) -> Vec<account::Model> {
    Account::find().filter(account::Column::PubKey.eq(pub_key))
    .all(get_db().as_ref()).await.unwrap()
}

pub async fn get_account_by(pub_key: &String) -> Result<account::Model, anyhow::Error> {
    let accounts = Account::find().filter(account::Column::PubKey.eq(pub_key))
    .all(get_db().as_ref()).await?;
    if accounts.len() == 0 {
        return Err(anyhow!("No Account"))
    }
    let account = accounts.get(0);
    Ok(account.unwrap().clone())
}

// /// 根据wallet_address获取账户信息
// pub async fn find_by_wallet_address(wallet_address: &String) -> Vec<account::Model> {
//     Account::find().filter(account::Column::WalletAddress.eq(wallet_address))
//     .all(get_db().as_ref()).await.unwrap()
// }