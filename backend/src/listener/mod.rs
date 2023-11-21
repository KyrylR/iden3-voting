use std::error::Error;
use std::sync::Arc;

use ethers::abi::Address;
use ethers::providers::{Http, Provider, StreamExt};

use crate::config::{make, Settings};
use crate::voting::{AddedCommitmentFilter, Voting};

pub async fn test_listen() -> Result<(), Box<dyn Error>> {
    let config = make("Settings.yaml")?;

    let contract = get_voting_instance(config.clone())?;

    let events = contract
        .event::<AddedCommitmentFilter>()
        .from_block(config.from_block);
    let mut stream = events.stream().await?;

    println!(
        "Listening for events; RPC URL: {}, contract address: {}, from block: {}",
        config.rpc_url, config.contract_address, config.from_block
    );

    while let Some(event) = stream.next().await {
        match event {
            Ok(f) => {
                println!("AddedCommitmentFilter event: {:?}", f);
            }
            Err(e) => {
                eprintln!("Error while listening for events: {:?}", e);
            }
        }
    }

    println!("Done listening for events");

    Ok(())
}

fn get_voting_instance(config: Settings) -> Result<Voting<Provider<Http>>, Box<dyn Error>> {
    let address = config.contract_address.parse::<Address>()?;
    let provider: Provider<Http> = Provider::<Http>::try_from(config.rpc_url)?;
    let client = Arc::new(provider);

    Ok(Voting::new(address, client))
}
