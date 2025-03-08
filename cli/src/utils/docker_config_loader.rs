use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug, Default)]
pub(crate) struct DockerConfig {
    auths: Option<HashMap<String, DockerAuthConfig>>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub(crate) struct DockerAuthConfig {
    pub auth: Option<String>,
    pub email: Option<String>,
}

impl DockerConfig {
    /// Loads the Docker configuration file (`config.json`) from the `.docker` directory.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the Docker configuration file, usually `~/.docker/config.json`.
    ///
    /// # Returns
    ///
    /// Returns a result containing the loaded `DockerConfig`, or an error if the operation fails.
    pub fn load(config_path: Option<String>) -> Result<DockerConfig> {
        let config_file_path = match config_path {
            Some(path) => Path::new(&path).to_path_buf(),
            None => {
                // Check if a DOCKER_CONFIG env is set and use that first
                let env_config_path_str = env::var("DOCKER_CONFIG");
                match env_config_path_str {
                    Ok(path) => Path::new(&path).to_path_buf(),
                    Err(_) => {
                        println!("DOCKER_CONFIG is not set, trying default path");
                        // Default to ~/.docker/config.json if no path is provided
                        let home_dir = env::var("HOME")
                            .with_context(|| "Failed to get HOME environment variable")?;
                        Path::new(&home_dir).join(".docker/config.json")
                    }
                }
            }
        };

        if !config_file_path.exists() {
            return Err(anyhow!(
                "Docker config file does not exist at {:?}",
                config_file_path
            ));
        }

        let config_data = fs::read_to_string(&config_file_path).with_context(|| {
            format!(
                "Failed to read Docker config file at {:?}",
                config_file_path
            )
        })?;
        let docker_config: DockerConfig =
            serde_json::from_str(&config_data).with_context(|| {
                format!(
                    "Failed to parse Docker config file at {:?}",
                    config_file_path
                )
            })?;

        Ok(docker_config)
    }

    /// Retrieves the authentication configuration for a specific registry.
    ///
    /// # Arguments
    ///
    /// * `registry_name` - The name of the registry (e.g., "docker.io").
    ///
    /// # Returns
    ///
    /// Returns an option containing the `AuthConfig` if found, or `None` if not.
    pub fn get_auth_config_for_registry(&self, registry_name: &str) -> Option<DockerAuthConfig> {
        self.auths
            .as_ref()
            .and_then(|auths| auths.get(registry_name).cloned())
    }
}
