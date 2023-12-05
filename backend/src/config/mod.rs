use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub rpc_url: String,
    pub contract_address: String,
    pub default_from_block: u64,
    pub abi_path: String,
    pub tree_height: u64,
}

pub fn make() -> Result<Settings, Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set");
    let contract_address =
        std::env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set");
    let default_from_block = std::env::var("DEFAULT_FROM_BLOCK")
        .expect("DEFAULT_FROM_BLOCK must be set")
        .parse::<u64>()?;
    let abi_path = std::env::var("ABI_PATH").expect("ABI_PATH must be set");
    let tree_height = std::env::var("TREE_HEIGHT")
        .expect("TREE_HEIGHT must be set")
        .parse::<u64>()?;

    Ok(Settings {
        rpc_url,
        contract_address,
        default_from_block,
        abi_path,
        tree_height,
    })
}
