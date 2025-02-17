use crate::utils::{tar_build_context, BuildDockerImageParams, BuildLog};
use anyhow::{anyhow, Result};
use bollard::image::BuildImageOptions;
use bollard::Docker;
use bytes::Bytes;
use futures_util::stream::StreamExt;
use std::sync::{Arc, Mutex};

/// Builds a Docker image using the provided build context tarball.
/// It will name the image local:imageHash.
///
/// # Arguments
/// * `docker` - A connected Docker client.
/// * `params` - Params from the user containing settings for the docker build.
/// * `image_hash` - The hash of the image.
/// * `build_log` - A shared, mutable reference to the build log.
///
/// # Returns
/// A `Result` indicating success or failure of the build process.
pub async fn build_image(
    docker: &Docker,
    params: Arc<BuildDockerImageParams>,
    image_hash: String,
    build_log: Arc<Mutex<BuildLog>>,
) -> Result<()> {
    // Create the build context tarball in a blocking task
    let build_context = {
        let build_log = Arc::clone(&build_log);
        let params_clone = Arc::clone(&params);
        tokio::task::spawn_blocking(move || {
            // Lock the Mutex to access the BuildLog
            let mut build_log = build_log.lock().unwrap();
            tar_build_context(&params_clone, &mut build_log)
        })
        .await?? // Handle both the JoinError and the Result from tar_build_context
    };

    // Set up build options
    let local_tag = format!("local:{}", image_hash);
    let build_options = BuildImageOptions {
        dockerfile: build_context
            .dockerfile_path
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("Dockerfile"), // Use the returned Dockerfile path or fallback to "Dockerfile"
        t: &local_tag, // Tag the image with the provided name
        rm: true,      // Remove intermediate containers after a successful build
        ..Default::default()
    };

    // Build the image
    let mut build_stream = docker.build_image(
        build_options,
        None,
        Some(Bytes::from(build_context.tarball)), // Convert Vec<u8> to Bytes
    );

    // Process the build output
    while let Some(output) = build_stream.next().await {
        match output {
            Ok(output) => {
                if let Some(message) = output.stream {
                    println!("{}", message);
                }
            }
            Err(e) => return Err(anyhow!("Build failed: {}", e)),
        }
    }

    Ok(())
}
