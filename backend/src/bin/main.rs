use std::error::Error;
use std::sync::Arc;

use axum::{
    response::{IntoResponse, Json},
    Router,
    routing::{get, post}, Server,
};
use ethers::utils::__serde_json::json;
use rs_merkle::MerkleTree;
use tokio::sync::Mutex;

use backend::{
    config::make,
    db::{fetch_all_commitments, fetch_max_block_number, get_database_pool, init_migration},
    handlers::{handle_proof, ProofRequest},
    listener::listen_commitments,
    poseidon_mt::{commitment_to_slice, hash_str, PoseidonHasher},
};
use serde_json::Value;

#[derive(Debug)]
struct ValidationError(String);

impl IntoResponse for ValidationError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({ "error": self.0 }));
        (axum::http::StatusCode::BAD_REQUEST, body).into_response()
    }
}

async fn build_and_init_mt() -> Result<(usize, Arc<Mutex<MerkleTree<PoseidonHasher>>>), Box<dyn Error>> {
    let pool = get_database_pool().await?;
    init_migration(&pool).await?;
    let max_block_number = fetch_max_block_number(&pool).await? as usize;
    let commitments = fetch_all_commitments(&pool).await?;

    let config = make()?;
    let initial_elements = vec![hash_str("0"); 2_u32.pow(config.tree_height as u32) as usize];
    let mut mt = MerkleTree::<PoseidonHasher>::from_leaves(initial_elements.as_slice());

    for commitment in commitments {
        mt.insert_with_index(commitment_to_slice(commitment.commitment));
    }

    Ok((max_block_number, Arc::new(Mutex::new(mt))))
}

async fn create_api(shared_mt: Arc<Mutex<MerkleTree<PoseidonHasher>>>) -> Router {
    Router::new()
        .route("/api", get(|| async { Json(json!({"message": "Hello, World!"})) }))
        .route("/api/proof", post(move |Json(payload): Json<ProofRequest>| {
            let mt_clone = shared_mt.clone();
            async move {
                if let Err(e) = payload.validate() {
                    return Err(ValidationError(e));
                }
                Ok(handle_proof(&payload, mt_clone).await)
            }
        }))
}

#[tokio::main]
async fn main() {
    let (from_block, shared_mt) = build_and_init_mt().await.expect("Failed to initialize Merkle Tree");

    tokio::spawn(listen_commitments(from_block, shared_mt.clone()));

    let app = create_api(shared_mt).await;

    Server::bind(&"0.0.0.0:3000".parse().expect("Failed to parse bind address"))
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
