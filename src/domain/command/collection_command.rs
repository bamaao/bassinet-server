#[derive(Debug)]
pub struct CreateCollectionCommand {
    pub title: String,
    pub description: String,
    pub is_public: u32,
    pub pub_key: String,
    pub icon_path: String,
}

#[derive(Debug)]
pub struct CreateArticleCommand {
    pub title: String,
    pub collection_id: String,
    pub description: String,
    pub is_public: u32,
    pub content: String,
    pub pub_key: String,
}

#[derive(Debug)]
pub struct AddVideoCommand {
    pub collection_id: String,
    pub title: String,
    pub description: String,
    pub is_public: u32,
    pub video_path: String,
    pub hash: String,
    pub pub_key: String,
}