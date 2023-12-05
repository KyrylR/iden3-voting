use std::error::Error;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{routing::get, Json, Router};
use ethers::utils::__serde_json::json;
use rs_merkle::MerkleTree;
use tokio::sync::Mutex;

use backend::config::make;
use backend::db::{
    fetch_all_commitments, fetch_max_block_number, get_database_pool, init_migration,
};
use backend::handlers::{handle_proof, ProofRequest};
use backend::listener::listen_commitments;
use backend::poseidon_mt::{commitment_to_slice, hash_str, PoseidonHasher};

#[derive(Debug)]
struct ValidationError(String);

impl IntoResponse for ValidationError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({ "error": self.0 }));
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

#[tokio::main]
async fn main() {
    println!("Building Merkle Tree...");

    let (from_block, mt) = init_mt().await.unwrap();

    let shared_mt = Arc::new(Mutex::new(mt));

    println!("Starting background task...");

    let listener_mt = shared_mt.clone();
    tokio::spawn(async move {
        let _ = listen_commitments(from_block, listener_mt)
            .await
            .map_err(|e| eprintln!("{:?}", e));
    });

    let handler_mt = shared_mt.clone();

    // build our application with a single route
    let app = Router::new()
        .route("/api", get(|| async { "Hello, World!" }))
        .route(
            "/api/proof",
            post(|Json(payload): Json<ProofRequest>| async move {
                if let Err(e) = payload.validate() {
                    return Err(ValidationError(e));
                }

                Ok(handle_proof(Json(payload), handler_mt).await)
            }),
        );

    println!("Starting server on localhost:3000...");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn build_mt() -> MerkleTree<PoseidonHasher> {
    let config = make().unwrap();

    let initial_elements = vec![hash_str("0"); 2_u32.pow(config.tree_height as u32) as usize];

    MerkleTree::<PoseidonHasher>::from_leaves(initial_elements.as_slice())
}

pub async fn init_mt() -> Result<(usize, MerkleTree<PoseidonHasher>), Box<dyn Error>> {
    match get_database_pool().await {
        Ok(pool) => {
            println!("Connected to database. Running migrations...");

            init_migration(&pool).await?;

            println!("Syncing Merkle Tree...");

            let mut mt = build_mt();

            let commitments = fetch_all_commitments(&pool).await?;
            let max_block_number = fetch_max_block_number(&pool).await? as usize;

            for commitment in commitments {
                mt.insert_with_index(commitment_to_slice(commitment.commitment));
            }

            Ok((max_block_number, mt))
        }
        Err(e) => {
            println!("Error connecting to database: {:?}", e);

            Ok((0, build_mt()))
        }
    }
}
