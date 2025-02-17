use oci_client::errors::OciDistributionError;
use oci_client::secrets::RegistryAuth;
use oci_client::{Client as RegistryClient, Reference};
use std::any::Any;

/// Checks if the registry contains an image with the tag specified. If the manifest check fails it
/// will return false. Otherwise, it will return true to indicate that the image does exist with the
/// tag given.
///
/// # Arguments
/// * `tag` - The image tag to check for.
/// * `reference` - An object containing details about the image repository and registry to perform
/// the check in.
/// * `registry_client` The authenticated OCI registry client to connect and check the registry with
///
/// # Returns
/// * `bool` A true or false flag indicating whether the tag already exists in the image repository.
///
pub async fn check_manifest_head(
    tag: &str,
    reference: Reference,
    registry_client: RegistryClient,
) -> bool {
    println!("Checking for the image hash {} on the registry.", tag);
    // Use anonymous here because the client should already be authenticated.
    return match registry_client
        .pull_image_manifest(&reference, &RegistryAuth::Anonymous)
        .await
    {
        Ok(manifest) => true,
        Err(error) => {
            eprintln!(
                "The image hash {} does not exist on the registry or we were unable to pull it.",
                tag
            );
            if error.type_id() == OciDistributionError::AuthenticationFailure.type_id() {
                println!("WARN: Unable to pull the details from the registry, please ensure you have the correct credentials.");
                println!("WARN: The build will continue, but this should investigated.");
            }
            eprintln!("{}", error);
            false
        }
    };
}
