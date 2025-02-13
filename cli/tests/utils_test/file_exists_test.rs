use crate::utils::file_exists;

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_exists() {
        // Create a temporary file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let file_path = temp_file.path().to_str().unwrap();

        // Check that the file exists
        assert!(file_exists(file_path), "File should exist");

        // Delete the file manually to test non-existence
        fs::remove_file(file_path).expect("Failed to delete temp file");

        // Check that the file no longer exists
        assert!(!file_exists(file_path), "File should not exist");
    }

    #[test]
    fn test_file_does_not_exist() {
        let non_existent_file = "some_random_non_existent_file.txt";
        assert!(!file_exists(non_existent_file), "File should not exist");
    }
}
