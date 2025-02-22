use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Version {
    pub version: String,
}

/// Parses a JSON byte slice into a `Version` struct.
///
/// # Arguments
/// * `json_data` - A byte slice containing JSON-encoded version information.
///
/// # Returns
/// * `Ok(Version)` if parsing is successful.
/// * `Err(serde_json::Error)` if parsing fails.
pub fn parse_version_file_json(json_data: &[u8]) -> serde_json::Result<Version> {
    serde_json::from_slice(json_data)
}
