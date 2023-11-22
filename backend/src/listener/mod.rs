use std::error::Error;
use std::sync::Arc;

use ethers::abi::Address;
use ethers::providers::{Http, Provider, StreamExt};
use ff::hex;
use rs_merkle::MerkleTree;
use tokio::sync::Mutex;

use crate::config::{make, Settings};
use crate::db::{get_database_pool, insert_one_commitment};
use crate::poseidon_mt::PoseidonHasher;
use crate::voting::{AddedCommitmentFilter, Voting};

pub async fn listen_commitments(
    from_block: usize,
    mt: Arc<Mutex<MerkleTree<PoseidonHasher>>>,
) -> Result<(), Box<dyn Error>> {
    let config = make("Settings.yaml")?;

    let contract = get_voting_instance(config.clone())?;

    let start_block = if from_block == 0 {
        config.default_from_block
    } else {
        from_block as u64
    };

    let events = contract
        .event::<AddedCommitmentFilter>()
        .from_block(start_block);
    let mut stream = events.stream().await?;

    println!(
        "Listening for events; RPC URL: {}, contract address: {}, from block: {}",
        config.rpc_url, config.contract_address, start_block
    );

    let pool = get_database_pool().await?;

    while let Some(event) = stream.next().await {
        match event {
            Ok(f) => {
                if (f.block_number.as_u32() as i32) <= from_block as i32 {
                    println!("Skipping event: {:?}", f);
                    continue;
                }

                println!("AddedCommitmentFilter event: {:?}", f);

                mt.lock().await.insert_with_index(f.commitment);
                insert_one_commitment(
                    &pool,
                    f.proposal_id.as_u32() as i32,
                    hex::encode(f.commitment).as_str(),
                    f.block_number.as_u32() as i32,
                )
                .await?;
            }
            Err(e) => {
                eprintln!("Error while listening for events: {:?}", e);
            }
        }
    }

    Ok(())
}

pub async fn test_listen() -> Result<(), Box<dyn Error>> {
    let config = make("Settings.yaml")?;

    let contract = get_voting_instance(config.clone())?;

    let events = contract
        .event::<AddedCommitmentFilter>()
        .from_block(config.default_from_block);
    let mut stream = events.stream().await?;

    println!(
        "Listening for events; RPC URL: {}, contract address: {}, from block: {}",
        config.rpc_url, config.contract_address, config.default_from_block
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
