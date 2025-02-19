use merkle_hash::blake3::Hasher;
use rayon::prelude::*;
use std::fs;
use std::io;

/// Hashes the given file with its filename included.
///
/// # Arguments
/// * `file` - The file to be hashed.
///
/// # Returns
/// * `Ok(String)` containing the hash if successful.
/// * `Err(io::Error)` if any file operation fails.
pub fn hash_file(file_path: &str) -> Result<String, io::Error> {
    fs::read(file_path).map(|content| {
        let mut hasher = Hasher::new();
        hasher.update(&content);
        hasher.update(file_path.as_bytes());
        hasher.finalize().to_hex().to_string()
    })
}

/// Hashes the given list of files in parallel and returns a combined hash.
///
/// # Arguments
/// * `watch_files` - A slice of file paths to be hashed.
///
/// # Returns
/// * `Ok(String)` containing the hash if successful.
/// * `Err(io::Error)` if any file operation fails.
pub fn hash_watch_files(watch_files: &[&str]) -> Result<String, io::Error> {
    if watch_files.is_empty() {
        return Ok(String::new());
    }

    let mut sorted_files = watch_files.to_vec();
    sorted_files.sort();

    let hashes: Vec<String> = sorted_files
        .par_iter()
        .filter_map(|file_path| match hash_file(file_path) {
            Ok(hash) => Some(hash),
            Err(_) => None,
        })
        .collect();

    if sorted_files.len() != hashes.len() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to hash one or more files",
        ));
    }

    let combined_hash_string = hashes.join("");
    let mut final_hasher = Hasher::new();
    final_hasher.update(combined_hash_string.as_bytes());
    Ok(final_hasher.finalize().to_hex().to_string())
}
