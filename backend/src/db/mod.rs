use std::error::Error;

use sqlx::{Connection, Pool, Postgres, sqlx_macros};

#[derive(Debug)]
pub struct Commitment {
    id: i64,
    proposal_id: i64,
    commitment: String,
    block_number: i64,
}

pub async fn fetch_all_commitments(
    pool: &Pool<Postgres>,
) -> Result<Vec<Commitment>, Box<dyn Error>> {
    let commitments = sqlx::query_as!(
        Commitment,
        r#"SELECT id, proposal_id, commitment, block_number FROM commitments"#
    )
    .fetch_all(pool)
    .await?;

    Ok(commitments)
}

#[sqlx_macros::test]
async fn test_query_as() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conn = sqlx::PgConnection::connect(database_url.as_str()).await?;

    let commitment_id = 1;
    let commitment = sqlx::query_as!(
        Commitment,
        r#"SELECT id, proposal_id, commitment, block_number FROM commitments WHERE id = $1"#,
        commitment_id
    )
    .fetch_one(&mut conn)
    .await?;

    println!("{:?}", commitment);

    Ok(())
}

#[sqlx_macros::test]
async fn test_insert_commitment() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conn = sqlx::PgConnection::connect(&database_url).await?;

    let new_proposal_id = 10; // Example proposal_id
    let new_commitment = "new_commitment_hash";
    let new_block_number = 12345; // Example block_number

    let inserted_commitment = sqlx::query!(
        r#"INSERT INTO commitments (proposal_id, commitment, block_number) VALUES ($1, $2, $3) RETURNING id, proposal_id, commitment, block_number"#,
        new_proposal_id, new_commitment, new_block_number
    )
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(inserted_commitment.proposal_id, new_proposal_id);
    assert_eq!(inserted_commitment.commitment, new_commitment);
    assert_eq!(inserted_commitment.block_number, new_block_number);

    Ok(())
}
