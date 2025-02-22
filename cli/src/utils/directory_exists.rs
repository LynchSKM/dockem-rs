use std::path::Path;

/// Checks if the path given is a directory that exists.
///
/// # Arguments
/// * `directory_path` - The directory path to check for existence.
///
/// # Returns
/// * `bool` A true or false value determining the directory's existence.
pub fn directory_exists(directory_path: &str) -> bool {
    let path = Path::new(directory_path);
    path.exists() && path.is_dir()
}

#[cfg(test)]
mod tests {
    use crate::utils::directory_exists::directory_exists;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_folder_exists() {
        // Create a temporary file
        let temp_folder = TempDir::new().expect("Failed to create temp folder");
        let folder_path = temp_folder.path().to_str().unwrap();

        // Check that the folder exists
        assert!(directory_exists(folder_path), "Folder should exist");

        // Delete the folder manually to test non-existence
        fs::remove_dir(folder_path).expect("Failed to delete temp folder");

        // Check that the file no longer exists
        assert!(!directory_exists(folder_path), "Folder should not exist");
    }

    #[test]
    fn test_file_does_not_exist() {
        let non_existent_folder = "./some_random_non_existent_folder";
        assert!(
            !directory_exists(non_existent_folder),
            "Folder should not exist"
        );
    }
}
