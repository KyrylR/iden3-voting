use std::sync::Arc;

use axum::Json;
use ethers::utils::hex;
use rs_merkle::MerkleTree;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::config::{make, Settings};
use crate::poseidon_mt::PoseidonHasher;

#[derive(Deserialize, Serialize)]
pub struct ProofRequest {
    commitment: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProofResponse {
    contract_address: String,
    tree_height: u64,
    commitment: String,
    tree_root: String,
    proof_hashes: Vec<String>,
    proof_indices: Vec<usize>,
}

pub async fn handle_proof(
    Json(payload): Json<ProofRequest>,
    mt: Arc<Mutex<MerkleTree<PoseidonHasher>>>,
) -> Json<ProofResponse> {
    let config: Settings = make("Settings.yaml").unwrap();

    let mt = mt.lock().await;

    let root = hex::encode(mt.root().unwrap());
    let mut leaf_idx_to_prove: Vec<usize> = vec![0; 1];

    let mut commitment: String = payload.commitment;
    for (inx, leaf) in mt.leaves().unwrap().to_vec().iter().enumerate() {
        if let Some(leaf) = commitment.strip_prefix("0x") {
            commitment = leaf.to_string();
        }

        if hex::encode(leaf) == commitment {
            leaf_idx_to_prove[0] = inx;
            break;
        }
    }

    let proof = mt.proof_custom(&leaf_idx_to_prove);

    let response = ProofResponse {
        contract_address: config.contract_address,
        tree_height: config.tree_height,
        commitment,
        tree_root: root,
        proof_hashes: proof.proof_hashes_hex().to_vec(),
        proof_indices: proof
            .proof_indices()
            .to_vec()
            .iter()
            .map(|x| x % 2)
            .collect(),
    };

    Json(response)
}
