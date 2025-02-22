use std::fs;
use std::path::PathBuf;

/// A guard to ensure the temporary file is cleaned up.
pub struct FileGuard {
    path: PathBuf,
}

impl FileGuard {
    /// Create a new guard for the file at the given path.
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for FileGuard {
    /// Clean up the file when the guard goes out of scope.
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path); // Ignore errors during cleanup
    }
}
