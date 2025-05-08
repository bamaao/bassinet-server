use anyhow::Ok;
use chrono::{Local, NaiveDateTime};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{domain::{model::entity::account, repository::account_repository::find_by_pubkey}, infrastructure::database_connection::get_db, interface::rest::dto::logon::SignUpPayload};

/// 注册账户
pub async fn register_account(payload: &SignUpPayload) -> Result<String, anyhow::Error> {
    let account = account::ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        nick_name: Set(if !payload.nick_name.is_empty() {Some(payload.nick_name.clone())} else {None}),
        avatar: Set("favicon.svg".to_owned()),
        pub_key: Set(Some(payload.pub_key.clone())),
        created_time: Set(NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()).unwrap()),
        status: Set(Some(1)),
        ..Default::default()
    };
    let exist_accounts = find_by_pubkey(&payload.pub_key).await;
    if exist_accounts.is_empty() {
        let account = account.insert(get_db().as_ref()).await?;
        return Ok(account.id.to_string());
    }
    anyhow::bail!("账号已存在")  
}