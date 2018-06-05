use std::fs;
use std::path::Path;

use sha3::{Digest, Sha3_256};

use super::{Result};

/// Converts a slice of bytes to a String of hexidecimal
/// characters.
/// # Note
/// The output string uses UPPERCASE letters for to represent
/// `0xA`, `0xB`, `0xC`, `0xD`, `0xE`, and `0xF`
pub fn to_hex_string(bytes: &[u8]) -> String {
    let pairs: Vec<String> = bytes.iter()
                                .map(|b: _| format!("{:02X}", b))
                                .collect();
    return pairs.join("");
}

/// Hashes the file at the given path using SHA-3-256.
/// # Errors
/// This function will error if the given path causes read_to_string
/// to error.  This is most commonly due to the path not existing or
/// a lack of read permissions on the file.
pub fn compute_file_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    return Ok(to_hex_string(&Sha3_256::digest_str(&contents)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    /// Does a basic consistency check to make sure that the
    /// `to_hex_string()` function works as expected.
    pub fn test_to_hex_string() {
        let input = vec![0x00, 0xFF, 0x55, 0xAA];
        let expected_output = String::from("00FF55AA");
        let output = to_hex_string(&input);
        assert_eq!(expected_output, output);
    }
    #[test]
    /// Writes a file with a known hash value, then runs
    /// `compute_file_hash()` on that path and compares it
    /// to the expected hash.
    pub fn test_file_hash() {
        let path = "/tmp/.file_hash_test.txt";
        let input = String::from("This is a test, please ignore");
        fs::write(path, input).unwrap();
        let expected_output = String::from("75715FF67FA39F9CB4EEF5912B8A0EE78AD5BB5D78B39BADC98DE4493E0F2AE5");
        let output = compute_file_hash(path).unwrap();
        fs::remove_file(path).unwrap();
        assert_eq!(expected_output, output);
    }
}