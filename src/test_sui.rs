
// // use ssi::json_ld::print;
// use sui_sdk::SuiClientBuilder;

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//     let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
//     println!("Sui testnet version: {}", sui_testnet.api_version());

//     let sui_devnet = SuiClientBuilder::default().build_devnet().await?;
//     println!("Sui devnet version: {}", sui_devnet.api_version());

//     let sui_mainnet = SuiClientBuilder::default().build_mainnet().await?;
//     println!("Sui mainnet version: {}", sui_mainnet.api_version());

//     // let sui_localnet = SuiClientBuilder::default().build_localnet().await?;
//     // println!("Sui localnet version: {}", sui_localnet.api_version());
//     Ok(())
// }