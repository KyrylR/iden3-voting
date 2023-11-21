use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub rpc_url: String,
    pub contract_address: String,
    pub from_block: u64,
    pub abi_path: String,
    pub tree_height: u64,
}

pub fn make(path: &str) -> Result<Settings, Box<dyn Error>> {
    let builder = Config::builder()
        .add_source(File::new(path, FileFormat::Yaml))
        .build()?;

    Ok(builder.try_deserialize()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File as FsFile;
    use std::io::Write;

    #[test]
    fn test_make() {
        // Create a temporary YAML file with test settings
        let yaml_content = r#"
        rpc_url: "http://localhost:8545"
        contract_address: "0x123..."
        from_block: 123456
        abi_path: "/path/to/abi.json"
        tree_height: 6
        "#;

        let path = "test_Settings.yaml";
        let mut file = FsFile::create(path).unwrap();
        write!(file, "{}", yaml_content).unwrap();

        // Run the make function
        let result = make(path);

        // Clean up the test file
        fs::remove_file(path).unwrap();

        // Check the results
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.rpc_url, "http://localhost:8545");
        assert_eq!(settings.contract_address, "0x123...");
        assert_eq!(settings.from_block, 123456);
        assert_eq!(settings.abi_path, "/path/to/abi.json");
        assert_eq!(settings.tree_height, 6);
    }
}
