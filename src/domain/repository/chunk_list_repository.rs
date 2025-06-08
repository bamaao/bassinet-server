use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::{domain::model::entity::{chunk_list::{self}, prelude::ChunkList}, infrastructure::database_connection::{self, get_db}};

/// 新增chunk_list
pub async fn add_chunk_list(chunk_list: chunk_list::ActiveModel) -> Result<(), anyhow::Error> {
    chunk_list.insert(database_connection::get_db().as_ref()).await?;
    Ok(())
}

/// 某md5的chunk list
pub async fn query_chunk_list(md5: &String) -> Vec<chunk_list::Model> {
    ChunkList::find().filter(chunk_list::Column::FileHash.eq(md5))
    .order_by_asc(chunk_list::Column::ChunkNumber)
    .all(get_db().as_ref()).await.unwrap()
}

pub async fn get_chunk(md5: &String, chunk_number: i32) -> Option<chunk_list::Model> {
    let chunks = ChunkList::find().filter(chunk_list::Column::FileHash.eq(md5).and(chunk_list::Column::ChunkNumber.eq(chunk_number)))
    .all(get_db().as_ref()).await.unwrap();

    if chunks.is_empty() {
        return None
    }
    Some(chunks.get(0).unwrap().clone())
}