use sui_sdk::types::base_types::ObjectID;

pub(crate) mod nft_query;

pub struct MyBassinetNft {
    pub object_id: ObjectID,
    pub object_type: String,
    pub package_id: String,
    pub module: String,
    pub name: String,
}