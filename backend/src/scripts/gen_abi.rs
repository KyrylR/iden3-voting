use ethers::contract::Abigen;

use backend::config::make;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = make("Settings.yaml").unwrap();

    Abigen::new("Voting", config.abi_path)?
        .generate()?
        .write_to_file("src/voting.rs")?;

    Ok(())
}
