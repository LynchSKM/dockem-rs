use merkle_hash::blake3::Hasher;
use merkle_hash::error::IndexingError;
use merkle_hash::{Algorithm, Encodable, MerkleTree};
use rayon::prelude::*;
use std::io;
use std::path::Path;

/// Hashes the given directory and its subdirectories and returns a combined hash.
///
/// # Arguments
/// * `watch_directories` - A slice of directory paths to be hashed.
///
/// # Returns
/// * `Ok(String)` containing the hash if successful.
/// * `Err(std::Error)` if any file operation fails.
pub fn hash_directory(directory: &str) -> Result<String, IndexingError> {
    let tree_result = MerkleTree::builder(directory)
        .algorithm(Algorithm::Blake3) // Change to preferred algorithm
        .hash_names(true) // Include file names in hash if needed
        .build();
    match tree_result {
        Ok(tree_final) => Ok(tree_final.root.item.hash.to_hex_string()),
        Err(error) => Err(error),
    }
}

/// Hashes the given list of directories in parallel, including their subdirectories, and returns a combined hash.
///
/// # Arguments
/// * `watch_directories` - A slice of directory paths to be hashed.
///
/// # Returns
/// * `Ok(String)` containing the hash if successful.
/// * `Err(io::Error)` if any file operation fails.
pub fn hash_watch_directories(watch_directories: &Vec<String>) -> Result<String, io::Error> {
    if watch_directories.is_empty() {
        return Ok(String::new());
    }

    let mut sorted_directories: Vec<String> = watch_directories
        .iter()
        .filter_map(|dir| Path::new(dir).to_str().map(|s| s.to_string()))
        .collect();
    sorted_directories.sort();

    let hashes: Vec<String> = sorted_directories
        .par_iter()
        .filter(|dir| Path::new(dir).is_dir())
        .filter_map(|directory| match hash_directory(directory) {
            Ok(hash) => Some(hash),
            Err(err) => {
                eprintln!("WARNING: Failed to hash directory '{}': {}", directory, err);
                None
            }
        })
        .collect();

    if hashes.len() != sorted_directories.len() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to hash one or more directories.",
        ));
    }

    let combined_hash_string = hashes.join("");
    let mut final_hasher = Hasher::new();
    final_hasher.update(combined_hash_string.as_bytes());
    Ok(final_hasher.finalize().to_hex().to_string())
}
