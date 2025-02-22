use super::docker_config_loader::DockerConfig;
use bollard::auth::DockerCredentials;
use bollard::Docker;

/// Creates a Docker client, either with explicit username/password credentials
/// or by loading authentication details from the Docker configuration file.
///
/// # Parameters
/// - `username`: An optional username to authenticate with the Docker registry.
/// - `password`: An optional password to authenticate with the Docker registry.
/// - `registry_name`: The name of the Docker registry (e.g., "docker.io" or a custom registry).
///
/// # Returns
/// - A `Result` containing a tuple with:
///   - `Docker`: The Docker client instance.
///   - `DockerCredentials`: The credentials used for authentication, which may be `None`
///     for direct credentials and contain the `auth` field for authentication when
///     using a configuration file.
///
/// # Errors
/// - If authentication fails or if no valid credentials are found, an error is returned.
///
/// This function will either create a Docker client using the provided credentials
/// or attempt to load credentials from the Docker configuration file (`~/.docker/config.json`).
/// If no credentials are provided, it tries to fetch them from the `auths` field in the Docker config.
pub async fn create_docker_client(
    username: Option<&str>,
    password: Option<&str>,
    registry_name: &str,
) -> Result<(Docker, DockerCredentials), Box<dyn std::error::Error>> {
    // Check if both username and password are provided
    if let (Some(user), Some(pass)) = (username, password) {
        // If credentials are provided, create a Docker client with the specified auth
        let auth = DockerCredentials {
            username: Some(user.to_string()),
            password: Some(pass.to_string()),
            auth: None,
            email: None,
            serveraddress: Some(registry_name.to_string()),
            identitytoken: None,
            registrytoken: None,
        };

        let docker = Docker::connect_with_socket_defaults()?;
        Ok((docker, auth))
    } else {
        // No credentials provided, so we load the Docker config file
        let docker_config = DockerConfig::load(None)?;

        // Attempt to get the auth config for the specified registry (or default registry)
        let auth_config = docker_config
            .get_auth_config_for_registry(&registry_name)
            .or_else(|| docker_config.get_auth_config_for_registry("docker.io"));

        if let Some(auth_config) = auth_config {
            let auth = DockerCredentials {
                username: None,
                password: None,
                auth: auth_config.auth,
                email: auth_config.email,
                serveraddress: Some(registry_name.to_string()),
                identitytoken: None,
                registrytoken: None,
            };

            let docker = Docker::connect_with_socket_defaults()?;
            Ok((docker, auth))
        } else {
            Err("No valid authentication configuration found.".into())
        }
    }
}
