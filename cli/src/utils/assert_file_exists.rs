use crate::assert_or_exit;
// Importing the macro
use crate::utils::file_exists;
use std::path::Path;

/// Asserts that a file exists at the given path. If not, either exits or returns an error message.
///
/// # Arguments
/// * `path` - The path to the file that should exist.
/// * `error_message` - The error message template. It can contain a `%s` for the file path.
///
/// # Returns
/// * `Ok(())` if the file exists.
/// * `Err(String)` containing the error message if the file does not exist (only in test mode).
pub fn assert_file_exists(path: &str, error_message: Option<&str>) -> Result<(), String> {
    let default_message = "ERROR: The file '%s' does not exist.";
    let error_message = error_message.unwrap_or(default_message);

    // Get absolute file path
    let abs_file_path = match Path::new(path).canonicalize() {
        Ok(abs_path) => abs_path.to_str().unwrap().to_string(),
        Err(_) => path.to_string(),
    };

    // Check if file exists
    let exists = file_exists(&abs_file_path);
    let output_message = error_message.replace("%s", &abs_file_path);

    assert_or_exit!(exists, output_message);
}

#[cfg(test)]
mod tests {
    use crate::utils::assert_file_exists;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_assert_file_exists() {
        // Create a temporary file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let file_path = temp_file.path().to_str().unwrap();

        // Check that the function returns Ok(()) when the file exists
        let result = assert_file_exists(file_path, None);
        assert!(result.is_ok(), "File should exist");

        // Delete the file manually to test non-existence
        fs::remove_file(file_path).expect("Failed to delete temp file");

        // Check that the function returns Err(String) when the file does not exist
        let result = assert_file_exists(file_path, None);
        assert!(result.is_err(), "File should not exist");
        assert_eq!(
            result.unwrap_err(),
            format!("ERROR: The file '{}' does not exist.", file_path)
        );
    }

    #[test]
    fn test_assert_file_does_not_exist() {
        let non_existent_file = "some_random_non_existent_file.txt";

        // Check that the function returns Err(String) when the file does not exist
        let result = assert_file_exists(non_existent_file, None);
        assert!(result.is_err(), "File should not exist");
        assert_eq!(
            result.unwrap_err(),
            format!("ERROR: The file '{}' does not exist.", non_existent_file)
        );
    }

    #[test]
    fn test_custom_error_message() {
        let non_existent_file = "another_random_non_existent_file.txt";
        let custom_message = "Custom Error: File '%s' is missing.";

        // Check custom error message is used
        let result = assert_file_exists(non_existent_file, Some(custom_message));
        assert!(result.is_err(), "File should not exist");
        assert_eq!(
            result.unwrap_err(),
            format!("Custom Error: File '{}' is missing.", non_existent_file)
        );
    }
}
