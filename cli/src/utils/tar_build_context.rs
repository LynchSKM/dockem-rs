use crate::utils::{BuildDockerImageParams, BuildLog, FileGuard};
use anyhow::{anyhow, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tar::{Builder, Header};

/// The result of creating the build context tarball.
pub struct TarBuildContextResult {
    pub(crate) tarball: Vec<u8>,                 // The gzipped tarball data
    pub(crate) dockerfile_path: Option<PathBuf>, // The relative path to the Dockerfile
    _dockerfile_guard: Option<FileGuard>,        // The guard for the temporary Dockerfile
}

/// Creates a gzipped tarball of the build context, including the Dockerfile and associated files.
///
/// # Arguments
/// * `params` - Params from the user containing settings for the docker build.
/// * `build_log` - An object containing build log parameters.
///
/// # Returns
/// A `Result` containing the tarball data and the relative path to the Dockerfile.
pub fn tar_build_context(
    params: &BuildDockerImageParams,
    build_log: &mut BuildLog,
) -> Result<TarBuildContextResult> {
    // Create path objects
    let context_path = Path::new(&params.directory);
    let dockerfile_path = Path::new(&params.dockerfile_path);

    // Ensure the context path is a directory
    if !context_path.is_dir() {
        return Err(anyhow!("Context path is not a directory"));
    }

    // Ensure the Dockerfile path is valid and relative to the context path
    if dockerfile_path.is_absolute() {
        return Err(anyhow!(
            "Dockerfile path must be relative to the context directory"
        ));
    }

    // Check if the Dockerfile is in a parent directory (e.g., ../Dockerfile)
    let (dockerfile_path_buf, dockerfile_guard) = if dockerfile_path.starts_with("../") {
        // Create a temporary Dockerfile in the build context directory
        let temp_dockerfile_path = context_path.join("Dockerfile");

        // Copy the contents of the original Dockerfile into the temporary file
        let mut original_dockerfile = File::open(dockerfile_path)?;
        let mut temp_file = File::create(&temp_dockerfile_path)?;
        io::copy(&mut original_dockerfile, &mut temp_file)?;

        // Mark that we're using a custom Dockerfile
        build_log.custom_dockerfile = true;

        // Create a guard to clean up the Dockerfile
        let guard = FileGuard::new(temp_dockerfile_path.clone());

        // Return the relative path to the Dockerfile and the guard
        (temp_dockerfile_path, Some(guard))
    } else {
        // Make sure Dockerfile is in context
        (dockerfile_path.to_path_buf(), None)
    };
    println!(
        "Creating tarball file with dockerfile path {:?}",
        dockerfile_path_buf
    );

    let mut tar_builder = Builder::new(Vec::new());
    let file_permission = 0o644;

    // Add the Dockerfile to the tarball
    let dockerfile_name = dockerfile_path_buf
        .file_name()
        .ok_or_else(|| anyhow!("Invalid Dockerfile path"))?
        .to_str()
        .ok_or_else(|| anyhow!("Invalid Dockerfile name"))?;
    let mut dockerfile = File::open(&dockerfile_path_buf)?;
    let mut header = Header::new_gnu();
    header.set_path(dockerfile_name)?;
    header.set_size(dockerfile.metadata()?.len());
    header.set_mode(file_permission); // Set file permissions
    header.set_cksum();
    tar_builder.append(&header, &mut dockerfile)?;

    // Add the rest of the build context
    for entry in std::fs::read_dir(context_path)? {
        let entry = entry?;
        let path = entry.path();

        // Skip directories
        if !path.is_file() {
            continue;
        }

        // Ensure the file path is valid and relative to the context directory
        let file_name = path
            .strip_prefix(context_path)
            .map_err(|_| anyhow!("File path escapes the context directory"))?
            .to_str()
            .ok_or_else(|| anyhow!("Invalid file name"))?;

        let mut file = File::open(&path)?;
        let mut header = Header::new_gnu();
        header.set_path(file_name)?;
        header.set_size(file.metadata()?.len());
        header.set_mode(file_permission); // Set file permissions
        header.set_cksum();
        tar_builder.append(&header, &mut file)?;
    }

    // Finish the tarball
    let tar_data = tar_builder.into_inner()?;

    // Compress the tarball using gzip (synchronous operation)
    let mut gz_encoder = GzEncoder::new(Vec::new(), Compression::default());
    gz_encoder.write_all(&tar_data)?;
    let gz_data = gz_encoder.finish()?;
    println!("Successfully compressed context into tarball");

    // Return the tarball data and the Dockerfile path
    Ok(TarBuildContextResult {
        tarball: gz_data,
        dockerfile_path: Some(dockerfile_path_buf.to_path_buf()),
        _dockerfile_guard: dockerfile_guard,
    })
}
