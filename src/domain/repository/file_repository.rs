use sea_orm::ActiveModelTrait;

use crate::{domain::model::entity::file_entity, infrastructure::database_connection};

/// 添加文件
pub async fn add_file(file_entity: file_entity::ActiveModel) -> Result<(), anyhow::Error> {
    file_entity.insert(database_connection::get_db().as_ref()).await?;
    Ok(())
}

/// 更新文件
pub async fn update_file(file_entity: file_entity::ActiveModel) -> Result<(), anyhow::Error> {
    todo!("更新文件");
}