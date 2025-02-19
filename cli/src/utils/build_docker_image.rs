use crate::utils::build_image::build_image;
use crate::utils::create_regclient_client::create_regclient_client;
use crate::utils::extract_version::extract_version;
use crate::utils::{
    check_manifest_head, copy_existing_image_tag, create_docker_client, generate_docker_image_name,
    hash_directory, hash_file, hash_string, hash_watch_directories, hash_watch_files,
    remove_empty_strings, tag_and_push_image, tag_and_push_new_images,
};
use crate::utils::{BuildDockerImageParams, BuildLog};
use anyhow::{anyhow, Context, Result};
use oci_client::secrets::RegistryAuth;
use std::sync::{Arc, Mutex};
use tokio::task;

/// Builds a Docker image or reuses an existing one based on content hashing.
///
/// # Arguments
/// * `params` - The parameters for building the Docker image.
///
/// # Returns
/// * `BuildLog` containing metadata about the build process.
pub async fn build_docker_image(mut params: BuildDockerImageParams) -> Result<BuildLog> {
    let mut build_log = BuildLog::default();

    // Filter out empty tags
    remove_empty_strings(&mut params.tag);
    let docker_username = params
        .docker_username
        .unwrap_or_else(|| "".parse().unwrap());
    let docker_password = params
        .docker_password
        .unwrap_or_else(|| "".parse().unwrap());

    // Compute overall hash in a blocking thread
    let watch_file_and_dir_hash = task::spawn_blocking(move || -> Result<String> {
        let mut hash_accumulator = String::new();

        // Hash watch files
        if let Some(watch_files) = &params.watch_file {
            hash_accumulator.push_str(&hash_watch_files(watch_files)?);
        }

        // Hash watch directories
        if let Some(watch_dirs) = &params.watch_directory {
            hash_accumulator.push_str(&hash_watch_directories(watch_dirs)?);
        }

        // Hash the build directory if not ignored
        if !params.ignore_build_directory {
            hash_accumulator.push_str(&hash_directory(&params.directory)?);
        }

        // Hash the Dockerfile
        hash_accumulator.push_str(&hash_file(&params.dockerfile_path)?);

        // Final hash for the image
        Ok(hash_string(&hash_accumulator))
    })
    .await
    .context("Failed to compute overall hash")?;
    let overall_hash = watch_file_and_dir_hash?.clone().as_str();
    build_log.image_hash = overall_hash.parse()?;

    // Extract version from version file
    let extract_version_result = task::spawn_blocking(move || -> Result<String> {
        Ok(extract_version(&build_log.image_hash)?)
    })
    .await
    .context(format!(
        "Failed to extract version from {}",
        params.version_file
    ))?;
    let version = extract_version_result?.as_str().clone();
    build_log.version = version.to_string();

    // Generate the hashed image name
    let image_name =
        generate_docker_image_name(&params.registry, &params.image_name, &overall_hash);
    build_log.hashed_image_name = image_name.clone();

    let (registry_client, reference) = create_regclient_client(
        &params.registry,
        &docker_username,
        &docker_password,
        &image_name,
        &mut build_log,
    )
    .await?;

    // Check if image already exists
    if check_manifest_head(&image_name, reference, &registry_client).await {
        println!(
            "Image {} already exists on the registry. Copying tags...",
            image_name
        );
        copy_existing_image_tag(
            &params,
            &version,
            &image_name,
            &registry_client,
            &RegistryAuth::Anonymous,
            &mut build_log,
        )
        .await?;
    } else {
        println!(
            "Image {} does not exist on the registry. Building and pushing...",
            image_name
        );

        // Create Docker client
        let (docker_client, docker_credentials) = create_docker_client(
            Some(docker_username),
            Some(docker_password),
            &params.registry,
        )
        .await
        .map_err(|error| anyhow!(error.to_string()))?;

        // Build the image
        let local_tag = build_image(
            &docker_client,
            Arc::new(params),
            &overall_hash,
            Arc::new(Mutex::new(build_log)),
        )
        .await?;
        build_log.local_tag = local_tag.clone();

        println!("Docker build complete. Pushing image...");

        // Tag and push the hashed image
        tag_and_push_image(&docker_client, &local_tag, &image_name, &docker_credentials).await?;
        println!("Image {} pushed to registry.", image_name);

        // Tag and push additional images
        tag_and_push_new_images(
            &docker_client,
            &params,
            &version,
            &local_tag,
            &docker_credentials,
            &mut build_log,
        )
        .await?;
    }

    Ok(build_log)
}
