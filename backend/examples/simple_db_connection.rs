use std::env;
use std::error::Error;
use std::path::Path;

use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;

use backend::db::fetch_all_commitments;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to database");

    let migrator = Migrator::new(Path::new("src/db/migrations")).await?;

    let mut conn = pool.acquire().await?;

    println!("Running migration");

    // run migration
    migrator.run(&mut conn).await?;

    println!("Migration done");

    let all_commitments = fetch_all_commitments(&pool).await?;

    all_commitments.iter().for_each(|commitment| {
        println!("ID: {:?}", commitment.id);
        println!("Commitment: {:?}", commitment.commitment);
        println!("Proposal ID: {:?}", commitment.proposal_id);
        println!("Block Number: {:?}", commitment.block_number);
        println!("==========================")
    });

    Ok(())
}
