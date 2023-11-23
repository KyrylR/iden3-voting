use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use ethers::utils::__serde_json::json;
use ethers::utils::hex;
use rs_merkle::MerkleTree;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::config::{make, Settings};
use crate::poseidon_mt::PoseidonHasher;
use crate::utils::is_hexadecimal;

#[derive(Debug)]
pub struct NotFoundError(String);

impl IntoResponse for NotFoundError {
    fn into_response(self) -> Response {
        let body = Json(json!({ "error": self.0 }));
        (StatusCode::NOT_FOUND, body).into_response()
    }
}

#[derive(Deserialize, Serialize)]
pub struct ProofRequest {
    commitment: String,
}

impl ProofRequest {
    pub fn validate(&self) -> Result<(), String> {
        if !is_hexadecimal(self.commitment.clone().as_str()) {
            return Err("Invalid commitment: must be a hex string".into());
        }

        if self.commitment.starts_with("0x") {
            if self.commitment.len() != 66 {
                return Err(
                    "Invalid commitment: must be a hex string of length 66 (with '0x')".into(),
                );
            }
        } else if self.commitment.len() != 64 {
            return Err(
                "Invalid commitment: must be a hex string of length 64 (without '0x')".into(),
            );
        }

        Ok(())
    }
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
) -> Result<Json<ProofResponse>, NotFoundError> {
    let config: Settings = make("Settings.yaml").unwrap();

    let mt = mt.lock().await;

    let root = hex::encode(mt.root().unwrap());
    let mut leaf_idx_to_prove: i32 = -1;

    let mut commitment: String = payload.commitment;
    for (inx, leaf) in mt.leaves().unwrap().to_vec().iter().enumerate() {
        if let Some(leaf) = commitment.strip_prefix("0x") {
            commitment = leaf.to_string();
        }

        if hex::encode(leaf) == commitment {
            leaf_idx_to_prove = inx as i32;
            break;
        }
    }

    if leaf_idx_to_prove == -1 {
        return Err(NotFoundError("Commitment not found in Merkle Tree".into()));
    }

    let proof = mt.proof_custom(&[leaf_idx_to_prove as usize]);

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

    Ok(Json(response))
}
