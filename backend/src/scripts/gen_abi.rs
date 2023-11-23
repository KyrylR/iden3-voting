use ethers::contract::Abigen;
use std::path::Path;

use backend::config::make;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = make("Settings.yaml").unwrap();

    // Generation of contract bindings (currently, to create a docker image, you need to manually
    // change the generated file to the following lines)
    //         ::core::include_bytes!(
    //             "abi/Voting.json",
    //         );
    Abigen::new("Voting", Path::new(&config.abi_path).to_str().unwrap())?
        .generate()?
        .write_to_file("src/voting.rs")?;

    Ok(())
}
