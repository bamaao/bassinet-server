use std::{io, path::Path, sync::Arc};
use axum::{extract::{Multipart, State}, http::StatusCode, response::IntoResponse, BoxError, Json};
use tokio_util::io::StreamReader;
use tokio::{fs::{self, File}, io::{AsyncWriteExt, BufWriter}};
use futures::{Stream, TryFutureExt, TryStreamExt};
use axum::body::Bytes;

use crate::{application::{command_service::{chunk_list_application_service, file_application_service}, query_service::{chunk_list_query_service}}, domain::{command::file_command::{AddChunkListCommand, AddFileCommand}, repository::chunk_list_repository}, infrastructure::{image_util::image_type, jwt::Claims}, ServerConfig};

use super::dto::{file_entity::{FileEntityDTO, MultiFileEntityDTO}, media::{ChunkListDTO, MediaDTO}};

/// 上传文件
pub async fn upload_file(State(state): State<Arc<ServerConfig>>, _: Claims, mut multipart: Multipart) -> Result<Json<MultiFileEntityDTO>, (StatusCode, String)> {
    let mut dtos = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        // let data = field.bytes().await.unwrap();
        let extension = Path::new(&file_name).extension();
        let ext = if extension.is_none() {
            "".to_owned()
        }else {
            String::from(extension.unwrap().to_str().unwrap())
        };
        println!(
            "`{name}` (`{file_name}`: `{content_type}`: `{ext}`)"
        );
        let mut file_path = String::new();
        file_path.push_str(uuid::Uuid::new_v4().to_string().as_str());
        file_path.push_str(ext.as_str());
        stream_to_file(&file_path, field, &state).await?;

        let command = AddFileCommand {
            mime: content_type,
            file_name: file_name,
            description: Option::None,
            // length: 0,
            path: file_path.to_string(),
            hash: Option::None,
        };
        dtos.push(file_application_service::add_file(command).await.unwrap());
    }
    Ok(Json(MultiFileEntityDTO { files: dtos }))
}

async fn stream_to_file<S, E>(path: &str, stream: S, config: &ServerConfig) -> Result<(), (StatusCode, String)> 
where S: Stream<Item=Result<Bytes, E>>,
      E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }
    async {
        // Convert the stream into an `AsyncRead`
        let body_with_io_error = stream.map_err(io::Error::other);
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        let path = std::path::Path::new(&config.assets_path).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }.await
    .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}

pub async fn upload_icon_file(State(state): State<Arc<ServerConfig>>, _: Claims, mut multipart: Multipart) -> Result<Json<FileEntityDTO>, (StatusCode, String)> {
    let field = multipart.next_field().await.unwrap().unwrap();
    // let name = field.name().unwrap().to_string();
    let file_name = field.file_name().unwrap().to_string();
    let content_type = field.content_type().unwrap().to_string();
    // let data = field.bytes().await.unwrap();
    let extension = Path::new(&file_name).extension();
    if extension.is_none() {
        return Err((StatusCode::INSUFFICIENT_STORAGE, "请上传图片格式文件".to_owned()))
    }
    let ext = extension.unwrap().to_str().unwrap();
    let image_type  = image_type(ext);
    if image_type.is_none() {
        return Err((StatusCode::INSUFFICIENT_STORAGE, "请上传图片格式文件".to_owned()))
    }
    // println!(
    //     "`{name}` (`{file_name}`: `{content_type}`: `{ext}`)"
    // );
    if !content_type.contains("image") {
        return Err((StatusCode::INSUFFICIENT_STORAGE, "请上传图片格式文件".to_owned()))
    }
    let mut file_path = String::new();
    file_path.push_str(uuid::Uuid::new_v4().to_string().as_str());
    file_path.push_str(".");
    file_path.push_str(ext);
    stream_to_icon_file(&file_path, field, &state).await?;

    let command = AddFileCommand {
        mime: content_type,
        file_name: file_name,
        description: Option::None,
        // length: 0,
        path: file_path.to_string(),
        hash: Option::None,
    };
    let mut dto = file_application_service::add_file(command).await.unwrap();
    let url_prefix = state.assets_http_addr.clone();
    dto.url = Some(url_prefix + "/icons/" + &dto.path);
    Ok(Json(dto))
}

async fn stream_to_icon_file<S, E>(path: &str, stream: S, config: &ServerConfig) -> Result<(), (StatusCode, String)> 
where S: Stream<Item=Result<Bytes, E>>,
      E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }
    async {
        // Convert the stream into an `AsyncRead`
        let body_with_io_error = stream.map_err(io::Error::other);
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        let path = std::path::Path::new(&config.assets_path).join("icons").join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }.await
    .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

/// 上传视频文件
pub async fn upload_video_chunks(State(state): State<Arc<ServerConfig>>, _: Claims, mut multipart: Multipart) -> impl IntoResponse {
    let mut file_name = String::new();
    let mut total_chunks = 0;
    let mut chunk_number = 0;
    let mut chunk_size = 0;
    let mut md5 = String::new();
    let mut chunk_data = Vec::new();
    let mut content_type = String::new();

    while let Some(field) = match multipart.next_field().await {
        Ok(f) => f,
        Err(_) => {
            return StatusCode::BAD_REQUEST
        }
    } {
        let field_name = field.name().unwrap_or_default().to_string();
        let type_opt = field.content_type();
        if type_opt.is_some() {
            content_type = field.content_type().unwrap().to_string();
            println!("content_type:{}", &content_type);
        }
        match field_name.as_str() {
            "fileName" => file_name = sanitize_filename::sanitize(field.text().await.unwrap_or_default()),
            "totalChunks" => total_chunks = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "chunkNumber" => chunk_number = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "chunkSize" => chunk_size = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "md5" => md5 = field.text().await.unwrap_or_default(),
            "chunk" => chunk_data = field.bytes().await.unwrap_or_else(|_| Vec::new().into()).to_vec(),
            _ => {}
        }
    }
    println!("final content_type:{}", &content_type);
    
    if file_name.is_empty() || md5.is_empty() || chunk_data.is_empty() {
        return StatusCode::BAD_REQUEST
    }

    let extension = Path::new(&file_name).extension();
    if extension.is_none() {
        return StatusCode::INSUFFICIENT_STORAGE
    }
    let ext = extension.unwrap().to_str().unwrap();
    if ext != "mkv" && ext != "mp4" {
        return StatusCode::INSUFFICIENT_STORAGE
    }

    let is_valid_md5 = is_valid_md5(&md5);
    if !is_valid_md5 {
        return StatusCode::BAD_REQUEST
    }
    // if !content_type.contains("video") {
    //     return (StatusCode::INSUFFICIENT_STORAGE, "请上传视频格式文件".to_owned())
    // }
    let temp_dir = format!("{}/{}", state.medias_path, md5);
    fs::create_dir_all(&temp_dir).unwrap_or_else(|_| {}).await;
    let chunk_path = format!("{}/chunk_{}", temp_dir, chunk_number);
    let file_path = Path::new(&chunk_path);
    if file_path.exists() {
        if file_path.is_dir() {
            let _ = fs::remove_dir_all(file_path).await;
        }else {
            let _ = fs::remove_file(file_path).await;
        }
    }
    let mut file = File::create(&chunk_path).await.unwrap();
    let _ = file.write_all(&chunk_data).await;
    let command = AddChunkListCommand{
        file_hash: md5,
        chunk_number: chunk_number,
        chunk_size: chunk_size,
        file_name: file_name,
        total_chunks: total_chunks,
    };
    let _ = chunk_list_application_service::add_chunk_list(command).await;
    StatusCode::OK
}

fn is_valid_md5(md5: &String) -> bool {
    if md5.len() != 32 {
        return false
    }
    let md5_check  = hex::decode(&md5);
    if md5_check.is_err() {
        return false
    }
    true
}

/// 合并上传文件
pub async fn merge_chunk_list(State(state): State<Arc<ServerConfig>>, _: Claims, Json(payload): Json<MediaDTO>) -> Result<String, (StatusCode, String)> {
    let md5 = payload.file_hash;
    let is_valid_md5 = is_valid_md5(&md5);
    if !is_valid_md5 {
        return Err((StatusCode::BAD_REQUEST, "Invalid parameter".to_owned()))
    }

    let chunks = chunk_list_repository::query_chunk_list(&md5).await;
    if chunks.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "未知文件".to_owned()))
    }
    let chunk = chunks.get(0).unwrap();
    let total_chunks = chunk.total_chunks;
    if total_chunks != chunks.len() as i32 {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Chunk不完整".to_owned()))
    }
    // let file_path = Path::new(&state.medias_path).join(&md5);
    // let mut cnt = 0;
    // if file_path.exists() && file_path.is_dir() {
    //     let dirs = std::fs::read_dir(file_path);
    //     if dirs.is_err() {
    //         return Err((StatusCode::INTERNAL_SERVER_ERROR, "文件不存在".to_owned()))
    //     }
    //     for entry in dirs.unwrap() {
    //         let entry = entry.unwrap();
    //         if entry.metadata().unwrap().is_file() {
    //             cnt += 1;
    //         }
    //     }
    // }else {
    //     return Err((StatusCode::INTERNAL_SERVER_ERROR, "文件目录不存在".to_owned()))
    // }
    // if chunks.get(0).unwrap().total_chunks != cnt {
    //     return Err((StatusCode::INTERNAL_SERVER_ERROR, "Chunk不完整".to_owned()))
    // }
    let temp_dir = format!("{}/{}", &state.medias_path, &md5);
    let file_name = chunk.file_name.clone();
    let path = Path::new(&file_name);
    let extension = path.extension().unwrap().to_str().unwrap();
    let target_name = md5.clone() + "." + extension;
    let output_path = format!("{}/{}", &temp_dir, &target_name);
    let output = Path::new(&output_path);
    if !output.exists() {
        let mut output_file = File::create(&output_path).await.unwrap();
        for chunk_number in 0..total_chunks {
            let chunk_path = format!("{}/chunk_{}", &temp_dir, chunk_number);
            let chunk_data = fs::read(&chunk_path).await;
            let _ = output_file.write_all(&chunk_data.unwrap()).await;
        }
    }
    // fs::remove_dir_all(temp_dir).await?;
    Ok(format!("{}/{}", &md5, &target_name))
}

/// 检查上传分片
pub async fn check_chunks(_: Claims, Json(payload): Json<MediaDTO>) -> Result<Json<Vec<ChunkListDTO>>, (StatusCode, String)> {
    let md5 = payload.file_hash;
    let is_valid_md5 = is_valid_md5(&md5);
    if !is_valid_md5 {
        return Err((StatusCode::BAD_REQUEST, "Invalid parameter".to_owned()))
    }
    let dtos = chunk_list_query_service::query_chunk_list(&md5).await;
    Ok(Json(dtos))
    // if dtos.is_empty() {
    //     return Ok(Json(ChunkInfoDTO{
    //         upload_status: 0,
    //         chunk_sign_arr: Vec::new(),
    //     }))
    // }
    // let file_name = dtos.get(0).unwrap().file_name.clone();
    // let path = Path::new(&file_name);
    // let extension = path.extension().unwrap().to_str().unwrap();
    // let target_file = md5.clone() + "." + extension;
    // let output = Path::new(&state.medias_path).join(&md5).join(target_file);
    // if output.exists()
    // Ok(Json(ChunkInfoDTO {
    //     upload_status: 1,
    //     chunk_sign_arr: Vec::new()
    // }))
}

// /// 检查上传分片
// pub async fn check_chunks(State(state): State<Arc<ServerConfig>>, _: Claims, Json(payload): Json<MediaDTO>) -> Result<Json<Vec<ChunkListDTO>>, (StatusCode, String)> {
//     let md5 = payload.file_hash;
//     let is_valid_md5 = is_valid_md5(&md5);
//     if !is_valid_md5 {
//         return Err((StatusCode::BAD_REQUEST, "Invalid parameter".to_owned()))
//     }
//     let dtos = chunk_list_query_service::query_chunk_list(&md5).await;
//     Ok(Json(dtos))
//     // if dtos.is_empty() {
//     //     return Ok(Json(ChunkInfoDTO{
//     //         upload_status: 0,
//     //         chunk_sign_arr: Vec::new(),
//     //     }))
//     // }
//     // let file_name = dtos.get(0).unwrap().file_name.clone();
//     // let path = Path::new(&file_name);
//     // let extension = path.extension().unwrap().to_str().unwrap();
//     // let target_file = md5.clone() + "." + extension;
//     // let output = Path::new(&state.medias_path).join(&md5).join(target_file);
//     // if output.exists()
//     // Ok(Json(ChunkInfoDTO {
//     //     upload_status: 1,
//     //     chunk_sign_arr: Vec::new()
//     // }))
// }