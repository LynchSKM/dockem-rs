use anyhow::{Context, Result};
use oci_client::client::Client as RegistryClient;
use oci_client::secrets::RegistryAuth;
use oci_client::Reference;
use std::str::FromStr;

/// Copies a Docker image within the same registry or across registries.
/// If the source and destination are in the same registry, it uses manifest re-tagging
/// to optimize the copy process without pulling and pushing layers.
///
/// # Arguments
/// * `source_image` - The source image reference (e.g., "registry.example.com/repo/image:tag").
/// * `destination_image` - The destination image reference.
/// * `registry_client` - An authenticated instance of the OCI registry client.
/// * `cred` - The registry authentication credentials.
///
/// # Returns
/// * `Ok(())` if the image was successfully copied.
/// * `Err(anyhow::Error)` if an error occurred during the process.
pub async fn copy_docker_image(
    source_image: &str,
    destination_image: &str,
    registry_client: &RegistryClient,
    cred: &RegistryAuth,
) -> Result<()> {
    // Parse the source and destination image references
    let src_reference =
        Reference::from_str(source_image).context("Failed to parse source image reference")?;
    let dest_reference = Reference::from_str(destination_image)
        .context("Failed to parse destination image reference")?;

    // Check if the source and destination are in the same registry
    if src_reference.registry() == dest_reference.registry() {
        println!("Source and destination are in the same registry. Using manifest re-tagging...");

        // Fetch the manifest of the source image
        let (manifest, source_digest_hash) = registry_client
            .pull_manifest(&src_reference, cred)
            .await
            .context("Failed to pull source image manifest")?;

        // Check if destination image already exists and has the same digest
        if let Ok((_dest_manifest, destination_digest_hash)) =
            registry_client.pull_manifest(&dest_reference, cred).await
        {
            if source_digest_hash == destination_digest_hash {
                println!("Destination image already exists with the same digest. Skipping copy.");
                return Ok(());
            }
        }

        // Push the source manifest to the destination reference (Re-tagging)
        registry_client
            .push_manifest(&dest_reference, &manifest)
            .await
            .context("Failed to push image manifest to destination")?;

        println!("Image successfully copied within the same registry.");
    } else {
        println!("Source and destination are in different registries. Performing standard pull and push...");

        // Define accepted media types for pulling the image
        let accepted_media_types = vec![
            "application/vnd.oci.image.manifest.v1+json",
            "application/vnd.docker.distribution.manifest.v2+json",
        ];

        // Pull the source image layers and manifest
        let image_data = registry_client
            .pull(&src_reference, cred, accepted_media_types)
            .await
            .context("Failed to pull source image layers")?;
        println!(
            "Image layers successfully pulled from source registry. {}",
            src_reference.registry()
        );

        // Push the layers to the destination registry
        registry_client
            .push(
                &dest_reference,
                &image_data.layers,
                image_data.config,
                cred,
                image_data.manifest,
            )
            .await
            .context("Failed to push image layer to destination")?;
        println!("Image layers successfully copied between the registries.");
        // Push the manifest to the destination
        let (source_manifest, _) = registry_client
            .pull_manifest(&src_reference, cred)
            .await
            .context("Failed to pull source image layers")?;
        registry_client
            .push_manifest(&dest_reference, &source_manifest)
            .await
            .context("Failed to push image manifest to destination")?;

        println!("Image successfully copied across registries.");
    }

    Ok(())
}
