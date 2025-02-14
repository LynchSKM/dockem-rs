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
