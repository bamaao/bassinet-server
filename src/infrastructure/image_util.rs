use std::path::Path;
use tokio::fs;

pub fn image_type(extension: &str) -> Option<&str>{
    let extension = extension.to_lowercase();
    return if extension == "gif" {
        Some("image/gif")
    }else if extension == "png" {
        Some("image/png")
    }else if extension == "jpeg" {
        Some("image/jpeg")
    }else if extension == "jpg" {
        Some("image/jpeg")
    }else if extension == "svg" {
        Some("image/svg+xml")
    }else {
        None
    }
}

/// 根据图片生成缩略图
pub async fn make_thumbnail(file_path: &Path) -> Option<String> {
    let file_stem = file_path.file_stem().unwrap();
    let extension = file_path.extension().unwrap();
    let parent = file_path.parent().unwrap();
    let target = parent.join(file_stem.to_str().unwrap().to_string() + "_thumb." + extension.to_str().unwrap());
    let image_type = image_type(extension.to_str().unwrap()).unwrap();
    if image_type == "image/svg+xml" {
        let result = fs::copy(file_path, &target).await;
        if result.is_err() {
            return None
        }
    }else {
        let image = image::open(file_path);
        if image.is_err() {
            return Option::None
        }
        let thumb = image::imageops::thumbnail(&image.unwrap(), 100, 100);
        
        let result = thumb.save(&target);
        if result.is_err() {
            return Option::None
        }
    }
    Some(target.as_os_str().to_str().unwrap().to_string())
}