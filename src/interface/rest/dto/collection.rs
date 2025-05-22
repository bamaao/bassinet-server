use serde::{Deserialize, Serialize};

/// 专辑
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionDTO {
    // 名称
    pub title: String,
    // 描述
    pub description: String,
    // 是否公开
    pub is_public: u32,
    pub request_id: String,
    pub icon_path: String,
}

// ///图集
// #[derive(Debug, Deserialize)]
// pub struct ImageGalleryDTO {
//     // 名称
//     pub name: String,
//     // 专辑ID(uuid)
//     pub collection_id: String,
//     pub description: String,
//     pub request_id: String,
// }

///图文
#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleDTO {
    // 标题
    pub title: String,
    // 专辑ID(uuid)
    pub collection_id: String,
    pub description: String,
    pub content: String,
    // 文档类型，目前只支持Markdown
    pub content_type: String,
    pub request_id: String,
}

// ///视频
// #[derive(Debug, Deserialize)]
// pub struct VideoDTO {
//     // 标题
//     pub title: String,
//     // 专辑ID(uuid)
//     pub collection_id: String,
//     // 简要描述
//     pub description: String,
//     // 存储路径
//     pub store_path: String,
//     // 视频格式，目前只支持mp4
//     pub video_format: String,
//     pub request_id: String,
// }

// ///音频
// #[derive(Debug, Deserialize)]
// pub struct AudioDTO {
//     // 标题
//     pub title: String,
//     // 专辑ID(uuid)
//     pub collection_id: String,
//     pub description: String,
//     // 存储路径
//     pub store_path: String,
//     // 音频格式
//     pub audio_format: String,
//     pub request_id: String,
// }

// ///文件夹
// #[derive(Debug, Deserialize)]
// pub struct FolderDTO {
//     // 名称
//     pub name: String,
//     // 专辑ID(uuid)
//     pub collection_id: String,
//     pub description: String,
//     // 存储路径
//     pub store_path: String,
//     pub request_id: String,
// }

// ///文件
// #[derive(Debug, Deserialize)]
// pub struct FileDTO {
//     // 名称
//     pub name: String,
//     // 文件类型
//     pub mime_type: String,
//     // // 专辑ID(uuid)
//     // pub collection_id: String,
//     // 所属类别
//     pub category_id: String,
//     // 所属类别类型，文件夹,图集
//     pub category_type: String,
//     pub description: String,
//     // 存储路径
//     pub store_path: String,
//     pub request_id: String,
// }

#[derive(Debug, Serialize)]
pub struct CollectionListDTO {
    pub collections: Vec<CollectionSimpleDTO>,
}

#[derive(Debug, Serialize)]
pub struct CollectionSimpleDTO {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct CollectionPageDTOList {
    pub dtos: Vec<CollectionPageDTO>,
    pub page_info: PageInfo,
}

#[derive(Debug, Serialize)]
pub struct CollectionPageDTO {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_public: u8,
    pub listing: u8,
    pub created_time: u64,
    pub icon_url: Option<String>,
    pub nft: Option<NftInfo>
}

#[derive(Debug, Serialize)]
pub struct NftInfo {
    pub id: String,
    pub package_id: String,
    pub collection_url: String,
    pub limit: u64,
    pub minting_price: u64,
    pub rewards_quantity: u64,
    pub mint_id: String,
    pub policy_id: String,
    pub policy_cap_id: String,
    pub coin_id: String,
    pub coin_package_id: String,
    pub coin_treasury_lock_id: String,
    pub coin_admin_cap_id: String
}

#[derive(Debug, Serialize)]
pub struct PageInfo {
    #[serde(rename="totalItems")]
    pub total: u64,
    #[serde(rename="totalPages")]
    pub pages: u64,
}

/// 专辑信息(专辑本身信息和包含的图文信息)
#[derive(Debug, Serialize)]
pub struct CollectionInfoDTO {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_public: u8,
    pub listing: u8,
    pub created_time: u64,
    pub icon_url: Option<String>,
    pub nft: Option<NftInfo>,
    pub articles: Vec<ArticleInfoDTO>,
}

#[derive(Debug, Serialize)]
pub struct CollectionSimpleInfoDTO {
    pub id: String,
    pub title: String,
    pub description: String,
    pub collection_url: String,
}

///图文
#[derive(Debug, Serialize)]
pub struct ArticleInfoDTO {
    pub id: String,
    // 标题
    pub title: String,
    // 专辑ID(uuid)
    pub collection_id: String,
    pub description: String,
    pub content: String,
    // 文档类型，目前只支持Markdown
    pub content_type: String,
    pub created_time: u64,
}