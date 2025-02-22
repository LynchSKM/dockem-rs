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
pub async fn build_docker_image(params: Arc<BuildDockerImageParams>) -> Result<BuildLog> {
    let mut build_log = BuildLog::default();

    // Create a cleaned version of the parameters
    let cleaned_params = {
        let mut params_clone = (*params).clone();
        remove_empty_strings(&mut params_clone.tag);
        params_clone
    };

    // Use the cleaned parameters for the rest of the function
    let docker_username = cleaned_params.docker_username.as_deref().unwrap_or("");
    let docker_password = cleaned_params.docker_password.as_deref().unwrap_or("");

    // Compute overall hash in a blocking thread
    let watch_file_and_dir_hash = task::spawn_blocking({
        let cleaned_params_clone = cleaned_params.clone();
        move || -> Result<String> {
            let mut hash_accumulator = String::new();

            if let Some(watch_files) = &cleaned_params_clone.watch_file {
                hash_accumulator.push_str(&hash_watch_files(watch_files)?);
            }

            if let Some(watch_dirs) = &cleaned_params_clone.watch_directory {
                hash_accumulator.push_str(&hash_watch_directories(watch_dirs)?);
            }

            if !cleaned_params_clone.ignore_build_directory {
                hash_accumulator.push_str(&hash_directory(&cleaned_params_clone.directory)?);
            }

            hash_accumulator.push_str(&hash_file(&cleaned_params_clone.dockerfile_path)?);
            Ok(hash_string(&hash_accumulator))
        }
    })
    .await
    .context("Failed to compute overall hash")??;
    build_log.image_hash = watch_file_and_dir_hash.clone();

    // Extract version from version file
    let version = task::spawn_blocking({
        let cleaned_params_clone = cleaned_params.clone();
        println!(
            "Extracting version from file {}",
            cleaned_params_clone.version_file
        );
        move || -> Result<String> { Ok(extract_version(&cleaned_params_clone.version_file)?) }
    })
    .await
    .context("Failed to extract version from file".to_string())??; // Handle JoinError and Result
    build_log.version = version.clone();

    // Generate the hashed image name
    let image_name = generate_docker_image_name(
        &cleaned_params.registry,
        &cleaned_params.image_name,
        &build_log.image_hash,
    );
    build_log.hashed_image_name = image_name.clone();

    let (registry_client, reference) = match create_regclient_client(
        &cleaned_params.registry,
        &docker_username,
        &docker_password,
        &image_name,
        &mut build_log,
    )
    .await
    {
        Ok((registry_client, reference)) => (registry_client, reference),
        Err(e) => {
            eprintln!("Failed to create registry client: {}", e);
            return Err(anyhow!("Failed to create registry client"));
        }
    };

    // Check if image already exists
    if check_manifest_head(&image_name, reference, &registry_client).await {
        println!(
            "Image {} already exists on the registry. Copying tags...",
            image_name
        );
        copy_existing_image_tag(
            &cleaned_params,
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
            &cleaned_params.registry,
        )
        .await
        .map_err(|error| anyhow!(error.to_string()))?;
        println!("Docker client authenticated successfully.");

        // Build the image
        let local_tag = build_image(
            &docker_client,
            &cleaned_params,
            &build_log.image_hash,
            Arc::new(Mutex::new(build_log.clone())), // Clone build_log to avoid moving it
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
            &cleaned_params,
            &version,
            &local_tag,
            &docker_credentials,
            &mut build_log,
        )
        .await?;
    }
    Ok(build_log)
}
