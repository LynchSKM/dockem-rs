use crate::assert_or_exit;
use crate::utils::directory_exists;
use std::path::Path;

/// Asserts that a directory exists at the given path. If not, either exits or returns an error message.
///
/// # Arguments
/// * `path` - The path to the directory that should exist.
/// * `error_message` - The error message template. It should contain a `%s` for the directory path.
///
/// # Returns
/// * `Ok(())` if the directory exists.
/// * `Err(String)` containing the error message if the directory does not exist (only in test mode).
pub fn assert_directory_exists(path: &str, error_message: Option<&str>) -> Result<(), String> {
    let default_message = "ERROR: The directory '%s' does not exist.";
    let error_message = error_message.unwrap_or(default_message);

    // Get absolute directory path
    let abs_directory_path = match Path::new(path).canonicalize() {
        Ok(abs_path) => abs_path.to_str().unwrap().to_string(),
        Err(_) => path.to_string(),
    };

    // Check if directory exists
    let exists = directory_exists(&abs_directory_path);
    let output_message = error_message.replace("%s", &abs_directory_path);

    assert_or_exit!(exists, output_message);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::utils::assert_directory_exists;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_assert_director_exists() {
        // Create a temporary director
        let temp_directory = TempDir::new().expect("Failed to create temp directory");
        let directory_path = temp_directory.path().to_str().unwrap();

        // Check that the function returns Ok(()) when the directory exists
        let result = assert_directory_exists(directory_path, None);
        assert!(result.is_ok(), "Directory should exist");

        // Delete the directory manually to test non-existence
        fs::remove_dir(directory_path).expect("Failed to delete temp directory");

        // Check that the function returns Err(String) when the directory does not exist
        let result = assert_directory_exists(directory_path, None);
        assert!(result.is_err(), "Directory should not exist");
        assert_eq!(
            result.unwrap_err(),
            format!("ERROR: The directory '{}' does not exist.", directory_path)
        );
    }

    #[test]
    fn test_assert_directory_does_not_exist() {
        let non_existent_directory = "some_random_non_existent_directory.txt";

        // Check that the function returns Err(String) when the directory does not exist
        let result = assert_directory_exists(non_existent_directory, None);
        assert!(result.is_err(), "Directory should not exist");
        assert_eq!(
            result.unwrap_err(),
            format!(
                "ERROR: The directory '{}' does not exist.",
                non_existent_directory
            )
        );
    }

    #[test]
    fn test_custom_error_message() {
        let non_existent_directory = "another_random_non_existent_directory.txt";
        let custom_message = "Custom Error: Directory '%s' is missing.";

        // Check custom error message is used
        let result = assert_directory_exists(non_existent_directory, Some(custom_message));
        assert!(result.is_err(), "Directory should not exist");
        assert_eq!(
            result.unwrap_err(),
            format!(
                "Custom Error: Directory '{}' is missing.",
                non_existent_directory
            )
        );
    }
}
