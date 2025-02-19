use crate::utils::build_log::BuildLog;
use anyhow::Result;
use oci_client::client::{Client, ClientConfig, ClientProtocol};
use oci_client::errors::OciDistributionError;
use oci_client::secrets::RegistryAuth;
use oci_client::{Reference, RegistryOperation};
use std::str::FromStr;

/// Creates an OCI distribution client and authenticates with the specified registry.
///
/// # Arguments
/// * `registry` - The registry URL. E.g <AWS_ACCOUNT_ID>.dkr.ecr.eu-west-1.amazonaws.com
/// * `username` - The username for authentication.
/// * `password` - The password for authentication.
/// * `docker_image_name` - The name of the image in the registry in this format `org/image-name:hash`
/// * `build_log` - A mutable reference to the `BuildLog` struct to record the build state.
///
/// # Returns
/// * `Result<(Client, Reference), OciDistributionError>` containing the initialized and authenticated client that can pull and push images or an error if it fails.
pub async fn create_regclient_client(
    registry: &str,
    username: &str,
    password: &str,
    docker_image_name: &str,
    build_log: &mut BuildLog,
) -> Result<(Client, Reference), OciDistributionError> {
    let mut custom_host = false;

    if !registry.is_empty() {
        build_log.docker_registry = Some(registry.to_string());
        custom_host = true;
    }

    if !username.is_empty() {
        build_log.docker_username = Some(username.to_string());
        custom_host = true;
    }

    if !password.is_empty() {
        build_log.docker_password = Some(password.to_string());
        custom_host = true;
    }

    if custom_host && registry.is_empty() {
        // Use Docker default registry if only authentication details are provided
        build_log.docker_registry = Some("index.docker.io".to_string());
    }

    build_log.custom_host = custom_host;

    // Determine authentication method
    let registry_auth = if custom_host && !password.is_empty() {
        RegistryAuth::Basic(username.to_string(), password.to_string())
    } else {
        RegistryAuth::Anonymous
    };

    let client_config = ClientConfig {
        protocol: ClientProtocol::Https,
        ..Default::default()
    };

    let client = Client::new(client_config);

    // Attempt authentication with the registry
    // Construct a reference to an image in the registry
    let reference = Reference::from_str(&docker_image_name)?;

    // Authenticate to ensure the client is ready for use
    client
        .auth(&reference, &registry_auth, RegistryOperation::Pull)
        .await
        .map_err(|err| {
            eprintln!(
                "ERROR: Failed to authenticate with registry for pull operation: '{}': {}",
                build_log.docker_registry.clone().unwrap().to_string(),
                err
            );
            err
        })?;

    client
        .auth(&reference, &registry_auth, RegistryOperation::Push)
        .await
        .map_err(|err| {
            eprintln!(
                "ERROR: Failed to authenticate with registry for push operation: '{}': {}",
                build_log.docker_registry.clone().unwrap().to_string(),
                err
            );
            err
        })?;
    Ok((client, reference))
}
