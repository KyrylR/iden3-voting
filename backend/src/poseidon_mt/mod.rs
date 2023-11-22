use ethers::utils::hex;
use ff::{from_hex, PrimeField};
use poseidon_rs::{Fr, Poseidon};
use rs_merkle::Hasher;

#[derive(Clone)]
pub struct PoseidonHasher {}

impl Hasher for PoseidonHasher {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        let hasher = Poseidon::new();

        let mut data_fr: Vec<Fr> = Vec::new();

        for byte in data.chunks(32) {
            let hex_value: Fr = from_hex(hex::encode(byte).as_str()).unwrap();
            data_fr.push(hex_value);
        }

        let hash = hasher.hash(data_fr).unwrap();

        commitment_to_slice(hash.into_repr().to_string())
    }
}

pub fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, hex::FromHexError> {
    let trimmed_str = if let Some(stripped) = hex_str.strip_prefix("0x") {
        stripped
    } else {
        hex_str
    };

    // Convert the hex string to bytes
    hex::decode(trimmed_str)
}

pub fn hash_str(item: &str) -> [u8; 32] {
    let hasher = Poseidon::new();

    let encoded_element: Fr = Fr::from_str(item).unwrap();
    let element_hash = hasher.hash(vec![encoded_element]).unwrap();

    commitment_to_slice(element_hash.into_repr().to_string())
}

pub fn commitment_to_slice(commitment: String) -> [u8; 32] {
    let commitment_bytes = hex_to_bytes(commitment.as_str()).unwrap();

    let mut array = [0u8; 32];
    array.copy_from_slice(commitment_bytes.as_slice());
    array
}

#[cfg(test)]
mod tests {
    use rs_merkle::MerkleTree;

    use super::*;

    #[test]
    fn test_hash() {
        let hasher = Poseidon::new();

        let b1: Fr = Fr::from_str("0").unwrap();
        let hash_zero = hasher.hash(vec![b1]).unwrap();
        assert_eq!(
            hash_zero.into_repr().to_string(),
            "0x2a09a9fd93c590c26b91effbb2499f07e8f7aa12e2b4940a3aed2411cb65e11c"
        )
    }

    #[test]
    fn test_mt() {
        let zero_hash = hash_str("0");

        let tree_height = 6;
        let initial_elements = vec![zero_hash; 2_u32.pow(tree_height) as usize];

        let mut mt = MerkleTree::<PoseidonHasher>::from_leaves(initial_elements.as_slice());

        assert_eq!(
            hex::encode(mt.root().unwrap()),
            "01c08b39621c262350bc2ddca369a968a68750dacb269e7aa9915245eb0ec3f1"
        );

        mt.insert_with_index(hash_str("1"));
        mt.insert_with_index(hash_str("1"));
        mt.insert_with_index(hash_str("1"));

        assert_eq!(
            hex::encode(mt.root().unwrap()),
            "04ba88a40409299cec2f4f9050688cebc59e8f40a27ee49771c01994609644a6"
        );
    }
}
