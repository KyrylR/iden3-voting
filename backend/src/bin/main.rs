use axum::{Router, routing::get};
use rs_merkle::MerkleTree;

use backend::config::make;
use backend::poseidon_mt::{hash_str, PoseidonHasher};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/api", get(|| async { "Hello, World!" }));

    println!("Building Merkle Tree...");

    let _mt = build_mt();

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