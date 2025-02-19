use crate::utils::os_open;
use crate::utils::parse_version_file_json;
use std::io;
use std::io::Read;

/// Expects a path to a JSON file that contains a `version` key. This key is parsed and returned
/// with a `v` prefix.
/// Example v1.0.0
///
/// # Arguments
/// * `version_file_path` - The path to the json file containing the version to extract.
///
/// # Returns
/// * `Result<String, io::Error>` - The version as a String or an error if extraction fails.
pub fn extract_version(version_file_path: &str) -> Result<String, io::Error> {
    // Open the version file
    let mut version_file = os_open(version_file_path).map_err(|err| {
        format!(
            "Failed to open version file '{}': {}",
            version_file_path, err
        );
        err
    })?;

    // Read the file content into a byte vector
    let mut bytes = Vec::new();
    version_file.read_to_end(&mut bytes).map_err(|err| {
        format!(
            "Failed to read version file '{}': {}",
            version_file_path, err
        );
        err
    })?;

    // Parse the version from the JSON content
    let parsed_version = parse_version_file_json(&bytes).map_err(|err| {
        eprintln!(
            "ERROR: Failed to parse version file. '{}': {}",
            version_file_path, err
        );
        err
    })?;

    let version = "v".to_owned() + &parsed_version.version;
    println!("The version of the image being built is: {}", version);
    Ok(version)
}
