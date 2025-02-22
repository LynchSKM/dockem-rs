use crate::utils::{
    copy_docker_image, generate_docker_image_name, BuildDockerImageParams, BuildLog,
};
use anyhow::{Context, Result};
use oci_client::client::Client as RegistryClient;
use oci_client::secrets::RegistryAuth;

/// Copies an existing image tag to new tags or to `latest` or `main version` based on the flags.
///
/// # Arguments
/// * `params` - Build parameters containing registry, image name, and tagging options.
/// * `version` - The version to be appended to the tags.
/// * `image_name_with_hash` - The source image name with its hash.
/// * `registry_client` - An authenticated instance of the OCI registry client.
/// * `cred` - The registry authentication credentials.
/// * `build_log` - A mutable reference to the build log structure.
///
/// # Returns
/// * `Ok(())` if the image tags were successfully copied.
/// * `Err(anyhow::Error)` if an error occurred during the process.
pub async fn copy_existing_image_tag(
    params: &BuildDockerImageParams,
    version: &str,
    image_name_with_hash: &str,
    registry_client: &RegistryClient,
    cred: &RegistryAuth,
    build_log: &mut BuildLog,
) -> Result<()> {
    // Iterate over tags specified in parameters
    for tag in &params.tag {
        let tag_version = format!("{}-{}", tag, version);
        let target_image_name =
            generate_docker_image_name(&params.registry, &params.image_name, &tag_version);
        println!("Copying the image to the new tag: {}", target_image_name);

        copy_docker_image(
            image_name_with_hash,
            &target_image_name,
            registry_client,
            cred,
        )
        .await
        .with_context(|| format!("Failed to copy image to tag: {}", target_image_name))?;

        build_log.output_tags.push(target_image_name);
    }

    // If no tags are specified and neither latest nor main version are selected
    if params.tag.is_empty() && !params.latest && !params.main_version {
        let main_version_image_name =
            generate_docker_image_name(&params.registry, &params.image_name, version);
        println!(
            "WARN: No tags were specified and --latest flag not selected. Copying to main version: {}",
            main_version_image_name
        );

        copy_docker_image(
            image_name_with_hash,
            &main_version_image_name,
            registry_client,
            cred,
        )
        .await
        .with_context(|| {
            format!(
                "Failed to copy image to main version: {}",
                main_version_image_name
            )
        })?;

        build_log.output_tags.push(main_version_image_name);
    }

    // If the --latest flag is selected
    if params.latest {
        let latest_image_name =
            generate_docker_image_name(&params.registry, &params.image_name, "latest");
        println!(
            "You have selected the --latest flag. Copying to latest tag: {}",
            latest_image_name
        );

        copy_docker_image(
            image_name_with_hash,
            &latest_image_name,
            registry_client,
            cred,
        )
        .await
        .with_context(|| format!("Failed to copy image to latest tag: {}", latest_image_name))?;

        build_log.output_tags.push(latest_image_name);
    }

    // If the --main-version flag is selected
    if params.main_version {
        let main_version_image_name =
            generate_docker_image_name(&params.registry, &params.image_name, version);
        println!(
            "You have selected the --main-version flag. Copying to main version: {}",
            main_version_image_name
        );

        copy_docker_image(
            image_name_with_hash,
            &main_version_image_name,
            registry_client,
            cred,
        )
        .await
        .with_context(|| {
            format!(
                "Failed to copy image to main version: {}",
                main_version_image_name
            )
        })?;

        build_log.output_tags.push(main_version_image_name);
    }

    Ok(())
}
