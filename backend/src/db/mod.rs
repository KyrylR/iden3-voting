use std::error::Error;

use sqlx::postgres::PgPoolOptions;
use sqlx::{sqlx_macros, Connection, Pool, Postgres};

#[derive(Debug)]
pub struct Commitment {
    pub id: i64,
    pub proposal_id: i32,
    pub commitment: String,
    pub block_number: i32,
}

pub async fn get_database_pool() -> Result<Pool<Postgres>, Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

pub async fn init_migration(pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    let exists = check_table_exists(&pool.clone()).await?;

    if !exists {
        println!("Running migration");

        create_table(&pool.clone()).await?;

        assert!(check_table_exists(&pool.clone()).await?);
    }

    Ok(())
}

async fn check_table_exists(conn: &Pool<Postgres>) -> Result<bool, Box<dyn Error>> {
    let table_exists = sqlx::query!(
        r#"SELECT EXISTS (
            SELECT FROM information_schema.tables
            WHERE  table_schema = 'public'
            AND    table_name   = 'commitments'
        ) AS table_exists"#
    )
    .fetch_one(conn)
    .await?
    .table_exists
    .unwrap();

    Ok(table_exists)
}

async fn create_table(conn: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"CREATE TABLE commitments
        (
            id           SERIAL PRIMARY KEY,
            proposal_id  INT          NOT NULL,
            commitment   VARCHAR(255) NOT NULL,
            block_number INT       NOT NULL
        )"#,
    )
    .execute(conn)
    .await?;

    Ok(())
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

pub async fn fetch_max_block_number(pool: &Pool<Postgres>) -> Result<i32, Box<dyn Error>> {
    let max_block_number =
        sqlx::query!("SELECT MAX(block_number) as max_block_number FROM commitments")
            .fetch_one(pool)
            .await?
            .max_block_number;

    Ok(max_block_number.unwrap_or_default())
}

pub async fn insert_one_commitment(
    pool: &Pool<Postgres>,
    proposal_id: i32,
    commitment: &str,
    block_number: i32,
) -> Result<Commitment, Box<dyn Error>> {
    let inserted_commitment = sqlx::query_as!(
        Commitment,
        r#"INSERT INTO commitments (proposal_id, commitment, block_number) VALUES ($1, $2, $3) RETURNING id, proposal_id, commitment, block_number"#,
        proposal_id, commitment, block_number
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_commitment)
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

#[sqlx_macros::test]
async fn test_delete_commitment() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conn = sqlx::PgConnection::connect(&database_url).await?;

    let commitment_proposal_id = 10; // Example commitment_id

    let deleted_commitment = sqlx::query!(
        r#"DELETE FROM commitments WHERE proposal_id = $1 RETURNING id, proposal_id, commitment, block_number"#,
        commitment_proposal_id
    )
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(deleted_commitment.proposal_id, commitment_proposal_id);

    Ok(())
}
