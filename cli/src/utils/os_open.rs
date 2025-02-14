use std::fs::File;
use std::io::{self};

/// Returns a File object that can be read.
///
/// # Arguments
/// * `name` - A string slice containing the path to the file to open.
///
/// # Returns
/// * `Ok(File)` if file is located successfully.
/// * `Err(io::Error)` if file opening fails.
pub fn os_open(name: &str) -> io::Result<File> {
    File::open(name)
}
