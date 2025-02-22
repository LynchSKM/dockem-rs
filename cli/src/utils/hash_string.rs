use merkle_hash::blake3::Hasher;

/// Hashes the given string using Blake3
///
/// # Arguments
/// * `input` - The string to be hashed.
///
/// # Returns
/// * `String` containing the hash
pub fn hash_string(input: &str) -> String {
    let mut hasher = Hasher::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::hash_string;

    #[test]
    fn test_hash_string() {
        let input = "Hello, world!";
        let hash_result = hash_string(input);

        // Blake3 should consistently return the same hash for the same input
        assert_eq!(
            hash_result,
            "ede5c0b10f2ec4979c69b52f61e42ff5b413519ce09be0f14d098dcfe5f6f98d"
        );

        // Different input should produce a different hash
        let different_input = "Goodbye, world!";
        let different_hash_result = hash_string(different_input);
        assert_ne!(hash_result, different_hash_result);

        // Empty string should produce a known hash
        let empty_input = "";
        let empty_hash_result = hash_string(empty_input);
        assert_eq!(
            empty_hash_result,
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
    }
}
