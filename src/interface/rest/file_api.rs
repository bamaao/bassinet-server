use std::{io, path::Path, sync::Arc};
use axum::{extract::{Multipart, State}, http::StatusCode, Json, BoxError};
use tokio_util::io::StreamReader;
use tokio::{fs::File, io::BufWriter};
use futures::{Stream, TryStreamExt};
use axum::body::Bytes;

use crate::{application::command_service::file_application_service, domain::command::file_command::AddFileCommand, infrastructure::jwt::Claims, ServerConfig};

use super::dto::file_entity::MultiFileEntityDTO;

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
            length: 0,
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