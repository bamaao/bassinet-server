use sea_orm::ActiveValue::Set;

use crate::{domain::{command::file_command::AddFileCommand, model::entity::file_entity, repository::file_repository}, interface::rest::dto::file_entity::FileEntityDTO};

/// 添加文件
pub async fn add_file(command: AddFileCommand) -> Result<FileEntityDTO, anyhow::Error> {
    let uuid = uuid::Uuid::new_v4();

    let file_entity = file_entity::ActiveModel {
        id: Set(uuid),
        name: Set(command.file_name.clone()),
        mime: Set(command.mime.clone()),
        // length: todo!(),
        path: Set(Some(command.path.clone())),
        hash: Set(command.hash),
        // ipfs: todo!(),
        status: Set(Some(1)),
        ..Default::default()
    };
    let _ = file_repository::add_file(file_entity).await?;

    let dto = FileEntityDTO{
        id: uuid.to_string(),
        name: command.file_name,
        mime: command.mime,
        description: command.description,
        path: command.path,
        // 文件的url路径
        url: Option::None,
    };
    Ok(dto)
}