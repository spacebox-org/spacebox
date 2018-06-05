use std::fs;
use std::path::Path;

use sha3::{Digest, Sha3_256};

use super::{Result};

pub fn to_hex_string(bytes: &[u8]) -> String {
    let pairs: Vec<String> = bytes.iter()
                                .map(|b: _| format!("{:02X}", b))
                                .collect();
    return pairs.join("");
}

pub fn compute_file_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    return Ok(to_hex_string(&Sha3_256::digest_str(&contents)));
}

#[cfg(test)]
mod tests {
    use std::fs;
    #[test]
    pub fn test_to_hex_string() {
        let input = vec![0x00, 0xFF, 0x55, 0xAA];
        let expected_output = String::from("00FF55AA");
        let output = super::to_hex_string(&input);
        assert_eq!(expected_output, output);
    }
    #[test]
    pub fn test_file_hash() {
        let path = "/tmp/.file_hash_test.txt";
        let input = String::from("This is a test, please ignore");
        fs::write(path, input).unwrap();
        let expected_output = String::from("75715ff67fa39f9cb4eef5912b8a0ee78ad5bb5d78b39badc98de4493e0f2ae5").to_ascii_uppercase();
        let output = super::compute_file_hash(path).unwrap();
        assert_eq!(expected_output, output);
    }
}