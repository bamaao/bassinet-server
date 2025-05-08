
use anyhow::Ok;
use chrono::{Local, NaiveDateTime};
use sea_orm::ActiveValue::Set;

use crate::domain::{command::collection_command::{CreateArticleCommand, CreateCollectionCommand}, model::entity::{collection, collection_item, prelude::Collection}, repository::{account_repository::find_by_pubkey, collection_repository::{self, get_by_id, search_collection_by}}};

/// 创建专辑
pub async fn create_collection(command: CreateCollectionCommand) -> Result<String, anyhow::Error> {
    // TODO 参数校验
    let id = uuid::Uuid::new_v4();
    let collection_id = id.to_string();
    let exist_accounts = find_by_pubkey(&command.pub_key).await;
    if exist_accounts.is_empty() {
        return anyhow::bail!("未知账户");
    }
    let collections_by_title = search_collection_by(&command.title, &command.pub_key).await?;
    if collections_by_title.len() > 0 {
        return anyhow::bail!("专辑名称重复");
    }
    let account_id = exist_accounts.get(0).unwrap().id.to_string();
    let collect = collection::ActiveModel {
        id: Set(id),
        title: Set(command.title),
        description: Set(command.description),
        is_public: Set(command.is_public.try_into().unwrap()),
        author: Set(account_id),
        seq: Set(1),
        status: Set(1),
        created_time: Set(NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()).unwrap()),
        ..Default::default()
    };
    let _ = collection_repository::create_collection(collect).await?;
    Ok(collection_id)
}

/// 创建文章
pub async fn create_article(command: CreateArticleCommand) -> Result<String, anyhow::Error> {
    // TODO 参数校验
    let id = uuid::Uuid::new_v4();
    let article_id = id.to_string();
    let exist_accounts = find_by_pubkey(&command.pub_key).await;
    if exist_accounts.is_empty() {
        return anyhow::bail!("未知账户");
    }
    let account_id = exist_accounts.get(0).unwrap().id.to_string();
    let collection = collection_repository::get_my_collection_by_id(&command.collection_id, &account_id).await;
    if collection.is_none() {
        return anyhow::bail!("未知专辑");
    }
    
    let article = collection_item::ActiveModel {
        id: Set(id),
        collection_id: Set(command.collection_id),
        seq: Set(1),
        title: Set(Some(command.title)),
        description: Set(Some(command.description)),
        created_time: Set(NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()).unwrap()),
        is_public: Set(command.is_public.try_into().unwrap()),
        author: Set(account_id),
        content: Set(Some(command.content)),
        status: Set(Some(1)),
        category: Set("article".to_owned()),
        ..Default::default()
    };
    let _ = collection_repository::create_collection_item(article).await?;
    Ok(article_id)
}