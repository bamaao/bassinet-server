use std::str::FromStr;

use sui_sdk::{rpc_types::{Page, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery}, types::{base_types::{ObjectID, SuiAddress}, parse_sui_struct_tag}, SuiClient, SuiClientBuilder};

use super::MyBassinetNft;

/// 获取指定账户的所有bassinet NFT package_id:bassinet_nft::BassinetNFT
pub async fn query_bassinet_nfts(address: &String) -> Result<Vec<MyBassinetNft>, anyhow::Error> {
    let mut results: Vec<MyBassinetNft> = Vec::new();

    let client = get_client().await?;
    let address = SuiAddress::from_str(&address).unwrap();
    let options = SuiObjectDataOptions::new().with_type();
    let query = SuiObjectResponseQuery::new(None, Some(options));
    let limit = Some(100 as usize);
    let mut cursor = None;
    let mut has_next_page = true;
    while has_next_page {
        let result = client.read_api().get_owned_objects(address, Some(query.clone()), cursor, limit).await?;
        has_next_page = result.has_next_page;
        cursor = result.next_cursor;
        let mut parse_results = parse(result);
        if parse_results.len() > 0 {
            results.append(&mut parse_results);
        }
    }
    
    Ok(results)
}

fn parse(page: Page<SuiObjectResponse, ObjectID>) -> Vec<MyBassinetNft> {
    let mut results = Vec::new();
    if !page.data.is_empty() {
        for item in page.data {
            let data = item.data.unwrap();
            let object_type = data.object_type().unwrap().to_string();
            let object_id = data.object_id;
            let bassinet_object_type = bassinet_struct_tag(&object_type);
            match bassinet_object_type {
                Some((package_id, module, name)) => {
                    results.push(MyBassinetNft { object_id: object_id, object_type: object_type, package_id: package_id, module: module, name: name});
                },
                None => {},
            }
        }
    }
    results
}

fn bassinet_struct_tag(object_type: &String) -> Option<(String, String, String)> {
    let strs: Vec<&str> = object_type.split("::").collect();
    if strs.len() != 3 {
        return None
    }
    let module = strs.get(1).unwrap().to_string();
    let name = strs.get(2).unwrap().to_string();
    if module == "bassinet_nft" && name == "BassinetNFT" {
        return Some((strs.get(0).unwrap().to_string(), module, name))
    }
    None
}

async fn get_client() -> Result<SuiClient, anyhow::Error> {
    let client = SuiClientBuilder::default()
    // .ws_url("wss://sui-testnet-rpc.publicnode.com")
    // .build("https://sui-testnet-rpc.publicnode.com")
    .build("https://fullnode.testnet.sui.io:443")
    .await?;
    Ok(client)
}

/// 根据类型获取nft
pub async fn get_any_bassinet_nft_by(address: &String, package_id: &String) -> Result<Option<ObjectID>, anyhow::Error> {
    let client = get_client().await?;
    let address = SuiAddress::from_str(&address).unwrap();

    let mut tag_str = String::from(package_id);
    tag_str.push_str("::");
    tag_str.push_str("bassinet_nft");
    tag_str.push_str("::BassinetNFT");
    let tag = parse_sui_struct_tag(tag_str.as_str()).unwrap();

    let filter = SuiObjectDataFilter::StructType(tag);
    let query = SuiObjectResponseQuery::new(Some(filter), None);
    let limit = Some(1 as usize);
    let result = client.read_api().get_owned_objects(address, Some(query), None, limit).await?;
    if result.data.is_empty() {
        return Ok(None)
    }
    Ok(Some(result.data.get(0).unwrap().object_id().unwrap()))
}