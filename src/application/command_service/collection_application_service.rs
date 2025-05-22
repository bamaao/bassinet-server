
use std::path::PathBuf;

use anyhow::Ok;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use tokio::fs;

use crate::domain::{command::collection_command::{CreateArticleCommand, CreateCollectionCommand}, model::entity::{collection, collection_item}, repository::{account_repository::{self}, collection_repository::{self}}};

/// 创建专辑
pub async fn create_collection(command: CreateCollectionCommand, icon_file_path: &PathBuf, assets_path: &String) -> Result<String, anyhow::Error> {
    // TODO 参数校验
    let id = uuid::Uuid::new_v4();
    let collection_id = id.to_string().replace('-', "");
    let exist_accounts = account_repository::find_by_pubkey(&command.pub_key).await;
    if exist_accounts.is_empty() {
        anyhow::bail!("未知账户");
    }
    let collections_by_title = collection_repository::search_collection_by(&command.title, &command.pub_key).await?;
    if collections_by_title.len() > 0 {
        anyhow::bail!("专辑名称重复");
    }
    let account_id = exist_accounts.get(0).unwrap().id.to_string();
    // 复制文件
    let collection_dir = std::path::Path::new(&assets_path).join(&collection_id);
    if !collection_dir.exists() {
        let _ = fs::create_dir(collection_dir).await;
    }else {
        if collection_dir.is_file() {
            anyhow::bail!("无法创建专辑图片文件")
        }
    }
    let target_path = std::path::Path::new(&assets_path).join(&collection_id).join(&command.icon_path);
    let _ = fs::copy(icon_file_path, target_path).await;
    let collect = collection::ActiveModel {
        id: Set(id),
        title: Set(command.title),
        description: Set(command.description),
        is_public: Set(command.is_public.try_into().unwrap()),
        author: Set(account_id),
        seq: Set(1),
        status: Set(1),
        listing: Set(Some(0)),
        created_time: Set(Local::now().naive_utc()),
        icon_url: Set(Some("/".to_owned() + &collection_id + "/" + &command.icon_path)),
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
    let exist_accounts = account_repository::find_by_pubkey(&command.pub_key).await;
    if exist_accounts.is_empty() {
        anyhow::bail!("未知账户");
    }
    let account_id = exist_accounts.get(0).unwrap().id.to_string();
    let collection = collection_repository::get_my_collection_by_id(&command.collection_id, &account_id).await;
    if collection.is_none() {
        anyhow::bail!("未知专辑");
    }
    
    let article = collection_item::ActiveModel {
        id: Set(id),
        collection_id: Set(command.collection_id),
        seq: Set(1),
        title: Set(Some(command.title)),
        description: Set(Some(command.description)),
        created_time: Set(Local::now().naive_utc()),
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