use std::sync::Arc;

use axum::{routing::get, Json, Router};
use rs_merkle::MerkleTree;
use tokio::sync::Mutex;

use backend::config::make;
use backend::handlers::{handle_proof, ProofRequest};
use backend::listener::listen_commitments;
use backend::poseidon_mt::{hash_str, PoseidonHasher};

#[tokio::main]
async fn main() {
    println!("Building Merkle Tree...");

    let mt = build_mt();

    let shared_mt = Arc::new(Mutex::new(mt));

    println!("Starting background task...");

    let listener_mt = shared_mt.clone();
    tokio::spawn(async move {
        let _ = listen_commitments(listener_mt)
            .await
            .map_err(|e| eprintln!("{:?}", e));
    });

    let handler_mt = shared_mt.clone();

    // build our application with a single route
    let app = Router::new()
        .route("/api", get(|| async { "Hello, World!" }))
        .route(
            "/api/proof",
            get(|Json(payload): Json<ProofRequest>| async move {
                handle_proof(Json(payload), handler_mt).await
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
    let config = make("Settings.yaml").unwrap();

    let initial_elements = vec![hash_str("0"); 2_u32.pow(config.tree_height as u32) as usize];

    MerkleTree::<PoseidonHasher>::from_leaves(initial_elements.as_slice())
}
