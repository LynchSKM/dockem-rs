use bollard::auth::DockerCredentials;
use bollard::errors::Error as BollardError;
use bollard::image::{PushImageOptions, TagImageOptions};
use bollard::Docker;
use futures_util::stream::StreamExt;

/// Tags and pushes a Docker image to a registry.
///
/// # Arguments
/// * `docker` - A connected Docker client.
/// * `from_image` - The source image name (e.g., `my-image:latest`).
/// * `to_image` - The target image name (e.g., `my-registry/my-image:latest`).
/// * `credentials` - Docker credentials for authentication.
///
/// # Returns
/// A `Result` indicating success or failure of the tag and push operations.
pub async fn tag_and_push_image(
    docker: &Docker,
    from_image: &str,
    to_image: &str,
    credentials: DockerCredentials,
) -> Result<(), BollardError> {
    // Split `to_image` into repo and tag
    let (repo, tag) = match to_image.split_once(':') {
        Some((repo, tag)) => (repo, tag), // If `:` is found, split into repo and tag
        None => (to_image, "latest"),     // If no `:`, use `latest` as the default tag
    };

    // Tag the image
    let tag_options = TagImageOptions {
        repo,
        tag,
        ..Default::default()
    };
    docker.tag_image(from_image, Some(tag_options)).await?;

    // Push the image
    let push_options = PushImageOptions {
        tag,
        ..Default::default()
    };
    let mut push_stream = docker.push_image(repo, Some(push_options), Some(credentials));

    // Process the push output
    while let Some(output) = push_stream.next().await {
        match output {
            Ok(output) => {
                // Print the status, progress, or error if they exist
                if let Some(status) = output.status {
                    println!("Status: {}", status);
                }
                if let Some(progress) = output.progress {
                    println!("Progress: {}", progress);
                }
                if let Some(error) = output.error {
                    eprintln!("Error: {}", error);
                }
            }
            Err(e) => return Err(e), // Return the error if the push fails
        }
    }

    Ok(())
}
