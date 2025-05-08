// /// 根据id获取专辑 
// pub fn get_by_id(account_id: String) -> Option<crate::domain::model::entity::> {
//     Option::None
// }

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{domain::model::entity::{account, prelude::Account}, infrastructure::database_connection::get_db};

/// 根据pub_key获取账户信息
pub async fn find_by_pubkey(pub_key: &String) -> Vec<account::Model> {
    Account::find().filter(account::Column::PubKey.eq(pub_key))
    .all(get_db().as_ref()).await.unwrap()
}